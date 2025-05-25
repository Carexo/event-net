use std::sync::Arc;
use neo4rs::{query, Graph};
use crate::models::event::Event;
use crate::repo::{RepoError};
use crate::repo::RepoError::Other;

pub struct UserEventRepository {
    graph: Arc<Graph>
}

impl UserEventRepository {
    pub fn new(
        graph: Arc<Graph>
    ) -> UserEventRepository {
        UserEventRepository { graph }
    }

    pub async fn assign_user_to_event(
        &self,
        user_name: &str,
        event_id: u16
    ) -> Result<(), RepoError> {
        self.graph.run(
            query(
            "\
                MATCH (u:User {name: $n})\
                MATCH (e:Event {id: $id})\
                MERGE (u)-[:REGISTERED_TO]->(e)\
                "
            )
                .param("n", user_name)
                .param("id", event_id)
        ).await.map_err(|e| Other(e.to_string()))?;
        Ok(())
    }
    
    pub async fn unassign_user_from_event(
        &self,
        user_name: &str,
        event_id: u16
    ) -> Result<(), RepoError> {
        self.graph.run(
            query(
                "\
                MATCH (u:User {name: $n})\
                MATCH (e:Event {id: $id})\
                MATCH (u)-[r:REGISTERED_TO]->(e)\
                DELETE r\
                "
            )
                .param("n", user_name)
                .param("id", event_id)
        ).await.map_err(|e| Other(e.to_string()))?;
        Ok(())
    }

    pub async fn find_all_events_of_user(
        &self,
        user_name: &str,
    ) -> Result<Vec<Event>, RepoError> {
        let mut rows = self.graph.execute(
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
                "
            )
                .param("n", user_name)
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
}