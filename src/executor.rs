use crate::account::Account;
use crate::database::DataBase;
use crate::config;
use crate::hasher;
use crate::history::HistoryPart;
use crate::{commands::Commands, commands::CommandWrapper};

pub struct Executor {}

impl Executor {
    pub fn execute(command: Commands) {
        match command {
            Commands::CreateUser { id, fio, birthdate, password } => {
                let password2 = hasher::hash_string(password);

                let user = Account {
                    id,
                    birthdate,
                    name: fio,
                    password: password2,
                    ..Default::default()
                };
                
                DataBase::set_account(user);
            },
            Commands::ChangeBalance { of, amount, for_what } => {
                // Update user balance
                let old_balance = DataBase::get_balance(&of);
                println!("Of: {}, Amount: {}, For what: {}", of, amount, for_what);
                
                if let Some(old_balance) = old_balance {
                    if amount > old_balance as i128 {
                        println!("Insufficent balance")
                    }
                    let new_balance = (old_balance as i128 + amount) as u128;
                    DataBase::set_balance(&of, new_balance);
                    
                }

                // Update user history
                let time = crate::time::get_time();
                DataBase::add_history(&of.clone(), HistoryPart {
                    for_what,
                    id_user: of,
                    sum: amount,
                    time
                })
            },
            Commands::DeleteUser { id } => {
                DataBase::remove_accout(&id);
            },
            Commands::ChangePassword { of, to } => {
                if let Some(mut account) = DataBase::get_account(&of) {
                    account.password = hasher::hash_string(to);
                    DataBase::set_account(account)
                }
            }
        }
    }

    pub fn validate(command_wrapper: CommandWrapper) -> bool {
        if let Some(account) = DataBase::get_account(&command_wrapper.login) {
            if hasher::hash_string(command_wrapper.password) != account.password {
                return false
            }
            
            if !([6, 7].contains(&account.id.len())) {
                println!("The login len: {}", &account.id.len());
                return false
            }
            println!("Validate true");
            true
        } else {
            println!("Account not found error: {} not found", &command_wrapper.login);

            false
        }
    }
}
