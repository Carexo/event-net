use crate::models::event::Event;
use crate::repo::RepoError;
use crate::repo::RepoError::Other;
use neo4rs::{Graph, query};
use std::sync::Arc;

pub struct UserEventRepository {
    graph: Arc<Graph>,
}

impl UserEventRepository {
    pub fn new(graph: Arc<Graph>) -> UserEventRepository {
        UserEventRepository { graph }
    }

    pub async fn assign_user_to_event(
        &self,
        user_name: &str,
        event_id: u16,
    ) -> Result<(), RepoError> {
        self.graph
            .run(
                query(
                    "\
                MATCH (u:User {name: $n})\
                MATCH (e:Event {id: $id})\
                MERGE (u)-[:REGISTERED_TO]->(e)\
                ",
                )
                .param("n", user_name)
                .param("id", event_id),
            )
            .await
            .map_err(|e| Other(e.to_string()))?;
        Ok(())
    }

    pub async fn unassign_user_from_event(
        &self,
        user_name: &str,
        event_id: u16,
    ) -> Result<(), RepoError> {
        self.graph
            .run(
                query(
                    "\
                MATCH (u:User {name: $n})\
                MATCH (e:Event {id: $id})\
                MATCH (u)-[r:REGISTERED_TO]->(e)\
                DELETE r\
                ",
                )
                .param("n", user_name)
                .param("id", event_id),
            )
            .await
            .map_err(|e| Other(e.to_string()))?;
        Ok(())
    }

    pub async fn find_all_events_of_user(&self, user_name: &str) -> Result<Vec<Event>, RepoError> {
        let mut rows = self
            .graph
            .execute(
                query(
                    "\
                MATCH (u:User {name: $n})\
                MATCH (u)-[:REGISTERED_TO]->(e:Event)\
                OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)\
                RETURN
                   e.id               AS eventId,
                   e.name             AS eventName,
                   e.startDatetime    AS start,
                   collect(k.name)    AS keywords;\
                ",
                )
                .param("n", user_name),
            )
            .await
            .map_err(|e| Other(e.to_string()))?;

        let mut events = Vec::<Event>::new();

        while let Some(row) = match rows.next().await {
            Ok(r) => r,
            Err(e) => return Err(Other(e.to_string())),
        } {
            let event: Event = Event::from_row(&row).map_err(|e| Other(e.to_string()))?;
            events.push(event);
        }

        Ok(events)
    }

    pub async fn recommend_events_for_user_based_on_events_similarity(
        &self,
        user_name: &str,
    ) -> Result<Vec<Event>, RepoError> {
        let mut rows = self.graph.execute(
            query(
                "\
                MATCH (u:User {name: $n})-[:REGISTERED_TO]->(e:Event)-[:HAS]->(k:EventKeyword)<-[:HAS]-(other:Event WHERE other.startDatetime > datetime())
                WHERE NOT EXISTS((u)-[:REGISTERED_TO]->(other))
                WITH e, other, count(k) AS intersection
                WITH e, other, intersection,
                  [(e)-[:HAS]->(ek:EventKeyword) | ek.name] AS set1,
                  [(other)-[:HAS]->(ok:EventKeyword) | ok.name] AS set2
                WITH e, other, intersection, set1, set2,
                  set1+[x in set2 WHERE NOT x IN set1] AS union
                WITH DISTINCT other, ((1.0*intersection)/size(union)) AS jaccard, set2
                WITH other AS e, jaccard, set2 AS keywords
                WHERE jaccard > 0.5
                RETURN
                   e.id               AS eventId,
                   e.name             AS eventName,
                   e.startDatetime    AS start,
                   keywords
                ORDER BY jaccard DESC;
                "
            ).param("n", user_name)
        ).await.map_err(|e| Other(e.to_string()))?;
        let mut events = Vec::<Event>::new();
        while let Some(row) = match rows.next().await {
            Ok(r) => r,
            Err(e) => return Err(Other(e.to_string())),
        } {
            let event: Event = Event::from_row(&row).map_err(|e| Other(e.to_string()))?;
            events.push(event);
        }
        Ok(events)
    }

    pub async fn is_user_registered_to_event(
        &self,
        user_name: &str,
        event_id: u16,
    ) -> Result<bool, RepoError> {
        let mut rows = self
            .graph
            .execute(
                query(
                    "MATCH (u:User {name: $n}), (e:Event {id: $id})
                    RETURN EXISTS((u)-[:REGISTERED_TO]->(e)) AS isAttending",
                )
                .param("n", user_name)
                .param("id", event_id),
            )
            .await
            .map_err(|e| Other(e.to_string()))?;

        // Get the first (and only) row from the result
        let row = match rows.next().await {
            Ok(Some(row)) => row,
            Ok(None) => return Err(Other("No result returned from query".to_string())),
            Err(e) => return Err(Other(e.to_string())),
        };

        // Extract the boolean value from the "isAttending" column
        let is_attending: bool = row.get("isAttending").map_err(|e| Other(e.to_string()))?;

        Ok(is_attending)
    }
}
