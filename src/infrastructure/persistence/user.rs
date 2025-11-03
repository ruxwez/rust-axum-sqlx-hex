use crate::{domain::entities, infrastructure::database::postgres::PGPool};

pub struct UserRepository {
    db: PGPool,
}

pub fn new_user_repository(db: PGPool) -> UserRepository {
    UserRepository { db }
}

impl UserRepository {
    pub async fn create_user(
        &self,
        email: &str,
        phone: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<entities::User, sqlx::Error> {
        // Create a user in the database
        let new_user = sqlx::query_as!(
            entities::User,
            r#"
            INSERT INTO "user" (email, phone, first_name, last_name)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            email,
            phone,
            first_name,
            last_name
        )
        .fetch_one(&self.db)
        .await?;

        Ok(new_user)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Result<Option<(i32, String, String)>, String> {
        // Implementation for retrieving a user by ID from the database
        Ok(None)
    }
}
