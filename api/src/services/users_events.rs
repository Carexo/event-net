use std::sync::Arc;
use crate::repo::users_events::UserEventRepository;
use crate::services::events::EventService;
use crate::services::users::UserService;

pub struct UserEventService {
    user_service: Arc<UserService>,
    event_service: Arc<EventService>,
    user_event_repo: UserEventRepository
}

impl UserEventService {
    pub fn new(
        user_service: Arc<UserService>,
        event_service: Arc<EventService>,
        user_event_repo: UserEventRepository
    ) -> Self {
        Self { user_service, event_service, user_event_repo }
    }
}