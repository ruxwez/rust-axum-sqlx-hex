use std::sync::Arc;

use crate::api::factory::StateRepositories;

pub struct UserService {
    repos: Arc<StateRepositories>,
}

pub fn new_user_service(repos: Arc<StateRepositories>) -> UserService {
    UserService { repos }
}

impl UserService {
    pub fn create_user(&self) {}

    pub fn get_user(&self) {}
}
