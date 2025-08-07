use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: i32,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}