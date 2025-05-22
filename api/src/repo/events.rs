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
    Other(String)
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
            Err(e) => return Err(RepoError::Other(e.to_string()))
        } {
            match Event::from_row(&row) {
                Ok(event) => events_list.push(event),
                Err(e) => return Err(RepoError::ParseError(e.to_string()))
            }
        }

        Ok(events_list)
    }
}
