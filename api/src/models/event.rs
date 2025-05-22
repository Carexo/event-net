use std::error::Error;
use neo4rs::{Row};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub id: u16,
    pub name: String,
    pub start_datetime: String,
    pub keywords: Vec<String>
}

impl Event {
    pub fn from_row(row: &Row) -> Result<Self, Box<dyn Error>> {
        Ok(Event{
            id: row.get("eventId")?,
            name: row.get("eventName")?,
            start_datetime: row.get("start")?,
            keywords: row.get("keywords")?,
        })
    }

    pub fn display(&self) -> String {
        format!(
            "Event ID: {}\nName: {}\nStart: {}\nKeywords: {:?}",
            self.id, self.name, self.start_datetime, self.keywords
        )
    }
}