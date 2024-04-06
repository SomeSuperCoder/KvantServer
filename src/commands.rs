use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Commands {
    CreateUser { id: String, fio: String, birthdate: String, password: String },
    DeleteUser { id: String },
    ChangeBalance { of: String, amount: i128, for_what: String },
    ChangePassword { of: String, to: String },
    SendMail { to: String, data: Vec<u8> }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandWrapper {
    pub login: String,
    pub password: String,
    pub command: Commands
}
