use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ping {
    pub id: String,
    pub msg: String,
    pub ts: i64,
}