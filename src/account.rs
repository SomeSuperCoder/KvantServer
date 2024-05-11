use serde::{Serialize, Deserialize};
use crate::history::HistoryPart;

#[derive(Serialize, Deserialize, Default)]
pub struct Account {
    pub id: String,
    pub password: String,
    pub name: String,
    pub birthdate: String,
    pub balance: u128,
    pub creator: String,
    pub history: Vec<HistoryPart>
}
