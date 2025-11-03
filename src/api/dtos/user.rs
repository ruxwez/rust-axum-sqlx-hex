#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateUserPayload {
    pub email: String,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
}
