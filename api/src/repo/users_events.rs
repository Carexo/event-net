use std::sync::Arc;
use neo4rs::Graph;
use crate::repo::events::EventRepository;
use crate::repo::users::UserRepository;

pub struct UserEventRepository {
    graph: Arc<Graph>
}

impl UserEventRepository {
    pub fn new(
        graph: Arc<Graph>
    ) -> UserEventRepository {
        UserEventRepository { graph }
    }
}