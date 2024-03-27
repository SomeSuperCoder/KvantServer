use crate::account::Account;
use crate::database::DataBase;
use crate::hasher;
use crate::{commands::Commands, commands::CommandWrapper};

pub struct Executor {}

impl Executor {
    pub fn execute(command: Commands) {
        match command {
            Commands::CreateUser { id, fio, birthdate, password_hash } => {
                let user = Account {
                    id,
                    birthdate,
                    name: fio,
                    password: password_hash,
                    ..Default::default()
                };
                
                DataBase::set_account(user);
            },
            Commands::SetBalance { of, amount } => {
                DataBase::set_balance(&of, amount);
            },
            Commands::DeleteUser { id } => {
                DataBase::remove_accout(&id);
            }
        }
    }

    pub fn validate(command_wrapper: CommandWrapper) -> bool {
        if let Some(account) = DataBase::get_account(&command_wrapper.login) {
            if hasher::hash_string(command_wrapper.login) != account.password {
                return false
            }
            
            if account.id.len() != 6 {
                return false
            }

            true
        } else {
            false
        }
    }
}
