use crate::models::event::{Event, EventUpdate};
use neo4rs::{Graph, Result, Row, query};
use rocket::http::Status;
use std::sync::Arc;
use thiserror::Error;
use crate::repo::ApiError;
use crate::repo::events::EventRepoError::Other;

#[derive(Error, Debug)]
pub enum EventRepoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("Event not found with id: {0}")]
    NotFound(u16),

    #[error("Error parsing event: {0}")]
    ParseError(String),

    #[error("Error: {0}")]
    Other(String),
}

impl ApiError for EventRepoError {
    fn status(&self) -> Status {
        match self {
            EventRepoError::NotFound(_) => Status::NotFound,
            _ => Status::BadRequest,
        }
    }
}
pub struct EventRepository {
    graph: Arc<Graph>,
}

impl EventRepository {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph }
    }

    pub async fn find_by_id(&self, id: u16) -> Result<Event, EventRepoError> {
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
            Some(row) => {
                Event::from_row(&row).map_err(|e| EventRepoError::ParseError(e.to_string()))
            }
            None => Err(EventRepoError::NotFound(id)),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<Event>, EventRepoError> {
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
            Err(e) => return Err(EventRepoError::Other(e.to_string())),
        } {
            match Event::from_row(&row) {
                Ok(event) => events_list.push(event),
                Err(e) => return Err(EventRepoError::ParseError(e.to_string())),
            }
        }

        Ok(events_list)
    }

    pub async fn add(&self, event: EventUpdate) -> Result<Event, EventRepoError> {
        if let Err(msg) = event.validate() {
            return Err(EventRepoError::Other(msg));
        }

        let mut result = self
            .graph
            .execute(
                query(
                    "\
                // Find the max ID and increment by 1 for the new event
                MATCH (e:Event)
                WITH COALESCE(MAX(e.id), 0) + 1 AS newId

                // Create the event with the new ID
                CREATE (e:Event {
                    id: newId,
                    name: $eventName,
                    startDatetime: datetime($startDatetime)
                })

                // Add keywords
                WITH e
                UNWIND $keywords AS kw
                  MERGE (k:EventKeyword { name: kw })
                  MERGE (e)-[:HAS]->(k)

                // Return the created event
                RETURN
                   e.id               AS eventId,
                   e.name             AS eventName,
                   e.startDatetime    AS start,
                   collect(k.name)    AS keywords
                ",
                )
                .param("eventName", event.name)
                .param("startDatetime", event.start_datetime)
                .param("keywords", event.keywords),
            )
            .await?;

        match result.next().await? {
            Some(row) => {
                Event::from_row(&row).map_err(|e| EventRepoError::ParseError(e.to_string()))
            }
            None => Err(EventRepoError::Other("Can't create event".to_string())),
        }
    }

    pub async fn remove(&self, id: u16) -> Result<String, EventRepoError> {
        let exists = self.find_by_id(id).await.is_ok();
        if !exists {
            return Err(EventRepoError::NotFound(id));
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
            .await?
            .next()
            .await?;

        Ok(format!("Event with id {} successfully deleted", id))
    }

    pub async fn edit(&self, id: u16, event_update: EventUpdate) -> Result<Event, EventRepoError> {
        if let Err(msg) = event_update.validate() {
            return Err(EventRepoError::Other(msg));
        }

        let exists = self.find_by_id(id).await.is_ok();
        if !exists {
            return Err(EventRepoError::NotFound(id));
        }

        let mut result = self
            .graph
            .execute(
                query(
                    "\
            MATCH (e:Event { id: $eventId })
            SET
              e.name          = coalesce($eventName, e.name),
              e.startDatetime = coalesce(datetime($startDatetime), e.startDatetime)
            WITH e
            OPTIONAL MATCH (e)-[oldRel:HAS]->(oldK:EventKeyword)
            DELETE oldRel
            WITH e
            UNWIND $keywords AS kw
              MERGE (k:EventKeyword { name: kw })
              MERGE (e)-[:HAS]->(k)
            RETURN
               e.id               AS eventId,
               e.name             AS eventName,
               e.startDatetime    AS start,
               collect(k.name)    AS keywords;",
                )
                .param("eventId", id)
                .param("eventName", event_update.name)
                .param("startDatetime", event_update.start_datetime)
                .param("keywords", event_update.keywords),
            )
            .await?;

        match result.next().await? {
            Some(row) => {
                Event::from_row(&row).map_err(|e| EventRepoError::ParseError(e.to_string()))
            }
            None => Err(EventRepoError::NotFound(id)),
        }
    }

    pub async fn get_featured(&self) -> Result<Vec<Event>, EventRepoError> {
        let mut result = self
            .graph
            .execute(query(
                "MATCH (e:Event)
                    OPTIONAL MATCH (e)-[:HAS]->(k:EventKeyword)
                    RETURN
                        e.id               AS eventId,
                        e.name             AS eventName,
                        e.startDatetime    AS start,
                        collect(k.name)    AS keywords
                    LIMIT 3;",
            ))
            .await?;

        let mut events_list: Vec<Event> = Vec::new();

        while let Some(row) = match result.next().await {
            Ok(r) => r,
            Err(e) => return Err(EventRepoError::Other(e.to_string())),
        } {
            match Event::from_row(&row) {
                Ok(event) => events_list.push(event),
                Err(e) => return Err(EventRepoError::ParseError(e.to_string())),
            }
        }

        Ok(events_list)
    }

    pub async fn get_events_by_keywords(&self, keyword: Vec<String>) -> Result<Vec<Event>, EventRepoError> {
        let mut rows = self.graph.execute(
            query(r#"
                MATCH (e:Event)-[:HAS]->(k:EventKeyword)
                WITH e, COLLECT(k.name) AS keywords
                WHERE all(kw IN $kws WHERE kw IN keywords)
                RETURN
                    e.id               AS eventId,
                    e.name             AS eventName,
                    e.startDatetime    AS start,
                    keywords
            "#)
                .param("kws", keyword)
        ).await.map_err(|e| Other(e.to_string()))?;
        let mut events = Vec::<Event>::new();
        while let Some(row) = match rows.next().await {
            Ok(r) => r,
            Err(e) => return Err(Other(e.to_string())),
        } {
            let event: Event = Event::from_row(&row).map_err(|e|Other(e.to_string()))?;
            events.push(event);
        }
        Ok(events)
    }

    pub async fn get_events_keywords(&self) -> Result<Vec<String>, EventRepoError> {
        let mut rows = self.graph.execute(
            query(r#"
            MATCH (k:EventKeyword)
            RETURN k.name
            "#
            )
        ).await.map_err(|e| Other(e.to_string()))?;
        let mut keywords = Vec::<String>::new();
        while let Some(row) = match rows.next().await {
            Ok(r) => r,
            Err(e) => return Err(Other(e.to_string())),
        } {
            let keyword: String = row.get("k.name").map_err(|e| Other(e.to_string()))?;
            keywords.push(keyword);
        }
        Ok(keywords)
    }
}
