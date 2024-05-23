use crate::{account::Account, history::HistoryPart};
use std::{collections::HashMap, fs};

const ACCOUNTS_PATH: &'static str = "./kvant_db/accounts";
const RAITING_PATH: &'static str = "./kvant_db/raiting";

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

    pub fn get_accout_skeleton(id: &String) -> Option<Account> {
        let account = Self::get_account(id);
        if let Some(mut account) = account {
            if account.password.as_str() != "ef797c8118f02dfb649607dd5d3f8c7623048c9c063d532cc95c5ed7a898a64f" {
                account.password = String::new();
            }

            Some(account)
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
        format!("{}/{}", ACCOUNTS_PATH, id)
    }

    pub fn get_user_skeleton_list() -> HashMap<String, Account> {
        // name: id
        let mut hm = HashMap::new();

        for entry in fs::read_dir(ACCOUNTS_PATH).unwrap() {
            let entry = entry.unwrap();
            let id = entry.file_name();
            let id = String::from(id.to_str().unwrap());
            
            let mut account = Self::get_accout_skeleton(&id).unwrap();
            account.password = String::new();

            hm.insert(id, account);
        }

        hm
    }

    pub fn check_login_credentials(login: String, password: String) -> bool {
        if let Some(account) = Self::get_account(&login) {
            account.password == password
        } else {
            false
        }
    }

    pub fn get_account_list() -> Vec<Account> {
        let mut result = Vec::new();
        let dir = fs::read_dir(ACCOUNTS_PATH).unwrap();

        for file in dir {
            let file = file.unwrap();
            let text = fs::read_to_string(file.path()).unwrap();
            let account: Account = serde_json::from_str(text.as_str()).unwrap();
            result.push(account)
        }

        result
    }

    pub fn get_raiting(of: &String) -> u128 {
        let mut raiting = 0;

        if let Ok(data) = fs::read_to_string(Self::make_raiting_path(of.clone())) {
            raiting = data.parse().unwrap();
        }

        raiting
    }

    pub fn add_raiting(to: String, amount: u128) {
        let mut raiting: u128 = Self::get_raiting(&to);

        raiting += amount;

        fs::write(Self::make_raiting_path(to), raiting.to_string()).unwrap();
    }

    pub fn get_raiting_skeletons() -> HashMap<String, u128> {
        let mut result = HashMap::new();

        let dir = fs::read_dir(RAITING_PATH).unwrap();

        for file in dir {
            let file = file.unwrap();
            let text = fs::read_to_string(file.path()).unwrap();
            if let Some(account) = Self::get_account(&file.file_name().to_str().unwrap().to_string()) {
                let balance: u128 = text.parse().unwrap();
                let name = account.name;

                result.insert(name, balance);
            }
        }

        result
    }
    
    pub fn reset_raiting() {
        fs::remove_dir_all(RAITING_PATH).unwrap();
    }

    pub fn make_raiting_path(id: String) -> String {
        format!("{}/{}", RAITING_PATH, id)
    }
}
