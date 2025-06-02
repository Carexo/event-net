use crate::models::user::User;
use crate::repo::ApiError;
use crate::repo::users::UserRepository;
use crate::utils::api_response::{ApiResponse, PaginatedItemsResponse};
use crate::utils::pagination::{PaginatedResponse, PaginationParams};

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn get_one(&self, user_name: &str) -> ApiResponse<User> {
        match self.user_repo.find_one(user_name).await {
            Ok(user) => ApiResponse::success(user, "User found"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }

    pub async fn get_all(
        &self,
        pagination: Option<PaginationParams>,
    ) -> PaginatedItemsResponse<User> {
        let params = pagination.unwrap_or_default();

        match self.user_repo.find_all(params.page, params.limit).await {
            Ok((users, total)) => {
                let paginated = PaginatedResponse::new(users, total, &params);

                ApiResponse::paginated(paginated, "Users retrieved successfully")
            }
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }
}
