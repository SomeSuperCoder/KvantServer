use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HistoryPart {
    pub id_user: String,
    pub sum: i128,
    pub for_what: String,
    pub time: String
}
