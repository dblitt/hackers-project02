use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkin {
    pub pk: String,       // Partition Key: "USER#<UserId>"
    pub sk: String,       // Sort Key: "ORDER#<OrderId>"
    pub checkin_id: String,
    pub location: String,
    pub time: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckinRequest {
    pub user_id: String,
    pub location: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCheckinResponse {
    pub checkin_id: String,
    pub user_id: String,
    pub location: String,
}
