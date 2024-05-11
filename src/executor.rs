use crate::account::Account;
use crate::database::DataBase;
use crate::hasher;
use crate::history::HistoryPart;
use crate::send_email::send_mail;
use crate::{commands::Commands, commands::CommandWrapper};

pub struct Executor {}

impl Executor {
    pub fn execute(command: Commands) {
        match command {
            Commands::CreateUser { id, fio, birthdate, password, creator } => {
                let password2 = hasher::hash_string(password);

                let user = Account {
                    id,
                    birthdate,
                    name: fio,
                    password: password2,
                    creator,
                    ..Default::default()
                };
                
                DataBase::set_account(user);
            },
            Commands::ChangeBalance { of, amount, for_what } => {
                if amount == 0 {
                    return
                }
                // Update user balance
                let old_balance = DataBase::get_balance(&of);
                println!("Of: {}, Amount: {}, For what: {}", of, amount, for_what);
                
                if let Some(old_balance) = old_balance {
                    if amount < 0 {
                        if amount > old_balance as i128 {
                            println!("Insufficent balance");
                            return
                        }
                    }
                    let new_balance = (old_balance as i128 + amount) as u128;
                    DataBase::set_balance(&of, new_balance);
                    if amount > 0 {
                        DataBase::add_raiting(of.clone(), amount as u128);
                    }
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
            },
            Commands::SendMail { to, title_subject, data } => {
                let _ = send_mail(&to, title_subject.as_str(), data);
            },
            Commands::ResetRaiting => {
                DataBase::reset_raiting();
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
