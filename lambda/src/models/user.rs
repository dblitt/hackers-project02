use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub pk: String,   // Partition Key: "USER#<UserId>"
    pub sk: String,   // Sort Key: "METADATA"
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub name: String,
    pub email: String,
}

