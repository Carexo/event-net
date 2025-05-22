use crate::models::event::Event;
use neo4rs::{Graph, Result, Row, query};
use std::sync::Arc;
use thiserror::Error;

pub struct EventRepository {
    graph: Arc<Graph>,
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("Event not found with id: {0}")]
    NotFound(u16),

    #[error("Error parsing event: {0}")]
    ParseError(String),

    #[error("Error: {0}")]
    Other(String),
}
impl EventRepository {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph }
    }

    pub async fn find_by_id(&self, id: u16) -> Result<Event, RepoError> {
        let mut result = self
            .graph
            .execute(
                query(
                    "\
                    MATCH (e:Event {id: $id})
                    OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
                    RETURN
                        e.id               AS eventId,
                        e.name             AS eventName,
                        e.startDatetime    AS start,
                        collect(k.name)    AS keywords;",
                )
                .param("id", id),
            )
            .await?;

        match result.next().await? {
            Some(row) => Event::from_row(&row).map_err(|e| RepoError::ParseError(e.to_string())),
            None => Err(RepoError::NotFound(id)),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<Event>, RepoError> {
        let mut result = self
            .graph
            .execute(query(
                "
                    MATCH (e:Event)
                    OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
                    RETURN
                        e.id               AS eventId,
                        e.name             AS eventName,
                        e.startDatetime    AS start,
                        collect(k.name)    AS keywords;",
            ))
            .await?;

        let mut events_list: Vec<Event> = Vec::new();

        while let Some(row) = match result.next().await {
            Ok(r) => r,
            Err(e) => return Err(RepoError::Other(e.to_string())),
        } {
            match Event::from_row(&row) {
                Ok(event) => events_list.push(event),
                Err(e) => return Err(RepoError::ParseError(e.to_string())),
            }
        }

        Ok(events_list)
    }

    pub async fn add(&self, event: Event) -> Result<Event, RepoError> {
        let mut result = self
            .graph
            .execute(
                query(
                    "\
            MERGE (e:Event { id: $eventId })
            SET
              e.name          = $eventName,
              e.startDatetime = datetime($startDatetime)
            WITH e
            UNWIND $keywords AS kw
              MERGE (k:EventKeyword { name: kw })
              MERGE (e)-[:HAS]->(k)
            RETURN
               e.id               AS eventId,
               e.name             AS eventName,
               e.startDatetime    AS start,
               collect(k.name)    AS keywords;
               ",
                )
                .param("eventId", event.id)
                .param("eventName", event.name)
                .param("startDatetime", event.start_datetime)
                .param("keywords", event.keywords),
            )
            .await?;

        match result.next().await? {
            Some(row) => {
                println!("{:?}", row);
                Event::from_row(&row).map_err(|e| RepoError::ParseError(e.to_string()))
            }
            None => Err(RepoError::Other("Can't create event".to_string())),
        }
    }

    pub async fn remove(&self, id: u16) -> Result<String, RepoError> {
        let exists = self
            .graph
            .execute(
                query(
                    "MATCH (e:Event {id: $eventId}) RETURN count(e) AS count"
                )
                    .param("eventId", id),
            )
            .await?
            .next()
            .await?
            .map(|row| row.get::<i64>("count").unwrap_or(0))
            .unwrap_or(0);

        if exists == 0 {
            return Err(RepoError::NotFound(id));
        }

        let _ = self
            .graph
            .execute(
                query(
                    "\
                MATCH (e:Event { id: $eventId })
                DETACH DELETE e;
                ",
                )
                .param("eventId", id),
            )
            .await?;

        Ok(format!("Event with id {} successfully deleted", id))
    }
}
