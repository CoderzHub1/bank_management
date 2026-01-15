use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account{
    pub id: u32,
    pub username: String,
    pub password: String,
    pub balance: f64,
    pub status: bool,
}