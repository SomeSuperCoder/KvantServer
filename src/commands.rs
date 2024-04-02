use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Commands {
    CreateUser { id: String, fio: String, birthdate: String, password: String },
    DeleteUser { id: String },
    ChangeBalance { of: String, amount: i128, for_what: String }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandWrapper {
    pub login: String,
    pub password: String,
    pub command: Commands
}
