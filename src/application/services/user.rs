use std::sync::Arc;

use crate::{api::factory::StateRepositories, domain::entities};

pub struct UserService {
    repos: Arc<StateRepositories>,
}

pub fn new_user_service(repos: Arc<StateRepositories>) -> UserService {
    UserService { repos }
}

impl UserService {
    pub async fn create_user(
        &self,
        email: &str,
        phone: &str,
        first_name: &str,
        last_name: &str,
    ) -> Option<entities::User> {
        let user_repo = &self.repos.user;

        user_repo
            .create_user(email, phone, first_name, last_name)
            .await
            .ok()
    }
}
