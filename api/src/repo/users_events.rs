use std::sync::Arc;
use neo4rs::{query, Graph};
use crate::repo::{RepoError};

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
        ).await.map_err(|e| RepoError::Other(e.to_string()))?;
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
        ).await.map_err(|e| RepoError::Other(e.to_string()))?;
        Ok(())
    }
}