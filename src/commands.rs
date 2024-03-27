use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Commands {
    CreateUser { id: String, fio: String, birthdate: String, password_hash: String },
    DeleteUser { id: String },
    SetBalance { of: String, amount: u128  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandWrapper {
    pub login: String,
    pub password: String,
    pub command: Commands
}
