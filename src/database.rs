use crate::{account::Account, history::HistoryPart};
use std::{collections::HashMap, fs};

pub struct DataBase {}

impl DataBase {
    pub fn get_balance(of: &String) -> Option<u128> {
        if let Some(account) = Self::get_account(of) {
            Some(account.balance)
        } else {
            None
        }
    }

    pub fn get_history(of: &String) -> Option<Vec<HistoryPart>> {
        if let Some(account) = Self::get_account(of) {
            Some(account.history)
        } else {
            None
        }
    }

    pub fn set_balance(of: &String, amount: u128) {
        if let Some(mut account) = Self::get_account(of) {
            account.balance = amount;
            Self::set_account(account)
        }
    }

    pub fn add_history(to: &String, part: HistoryPart) {
        if let Some(mut account) = Self::get_account(&to) {
            account.history.push(part);
            Self::set_account(account)
        }
    }

    pub fn get_account(id: &String) -> Option<Account> {
        if let Ok(account_string) = fs::read_to_string(Self::make_account_path(id)) {
            serde_json::from_str(account_string.as_str()).unwrap()
        } else {
            None
        }
    }

    pub fn set_account(account: Account) {
        fs::write(Self::make_account_path(&account.id), serde_json::to_string(&account).unwrap()).unwrap()
    }

    pub fn remove_accout(id: &String) {
        let _ = fs::remove_file(Self::make_account_path(id));
    }

    pub fn does_exist(id: &String) -> bool {
        fs::read(Self::make_account_path(id)).is_ok()
    }

    pub fn make_account_path(id: &String) -> String {
        format!("./kvant_db/{}", id)
    }

    pub fn get_user_skeleton_list() -> HashMap<String, String> {
        // name: id
        let mut hm = HashMap::new();

        for entry in fs::read_dir("./kvant_db/").unwrap() {
            let entry = entry.unwrap();
            let id = entry.file_name();
            let id = String::from(id.to_str().unwrap());
            
            let name = Self::get_account(&id).unwrap().name;

            hm.insert(id, name);
        }

        hm
    }

    pub fn check_login_credentials(login: String, password_hash: String) -> bool {
        if let Some(account) = Self::get_account(&login) {
            account.password == password_hash
        } else {
            false
        }
    }
}
