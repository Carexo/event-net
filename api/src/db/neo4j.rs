use neo4rs::*;
use std::sync::Arc;

pub struct Neo4jConnection {
    pub graph: Arc<Graph>,
}

impl Neo4jConnection {
    pub async fn new(uri: &str, user: &str, password: &str) -> Result<Self, Error> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .build()?;

        let graph = Arc::new(Graph::connect(config).await?);

        Ok(Neo4jConnection { graph })
    }
}
