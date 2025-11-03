use std::sync::Arc;

use crate::infrastructure::persistence::user::UserRepository;

pub struct UserService {
    user_repo: Arc<UserRepository>,
}

pub fn new_user_service(user_repo: Arc<UserRepository>) -> UserService {
    UserService { user_repo }
}

impl UserService {
    pub fn create_user(&self) {
        // Logic to create a user
    }

    pub fn get_user(&self) {
        // Logic to get a user
    }
}
