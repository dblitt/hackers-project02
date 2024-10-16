use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Checkin {
    pub id: String,
    pub user: String,
    pub time: String,
    pub location: String,
}

impl Checkin {
    pub fn new(id: String, user: String, time: String, location: String) -> Self {
        Checkin { id, user, time, location }
    }
}
