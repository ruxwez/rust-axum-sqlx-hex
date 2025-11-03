// On this module, we will define the structs like a Body, Requests, Responses, Headers, etc.
pub mod user;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Response<T> {
    pub success: bool,
    pub message: Option<String>,
    pub result: Option<T>,
}
