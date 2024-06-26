pub mod commands;
pub mod executor;
pub mod history;
pub mod database;
pub mod account;
pub mod hasher;
pub mod time;
pub mod config;
pub mod send_email;

use database::DataBase;
use rocket::*;
use rocket::serde::json::Json;
use commands::*;
use executor::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, execute, get_account_skeleton_list, check_login_credentials, get_account, does_exist_pair, passwd, get_raiting_skeleton, get_raiting])
}

#[get("/")]
fn index() -> &'static str {
    "Kvant Server"
}

#[post("/execute", data = "<command_wrapper>")]
fn execute(command_wrapper: Json<CommandWrapper>) -> &'static str {
    let command_wrapper = command_wrapper.into_inner();

    if Executor::validate(command_wrapper.clone()) {
        Executor::execute(command_wrapper.command)
    }

    ""
}

#[get("/get_account_skeleton_list")]
fn get_account_skeleton_list() -> String {
    serde_json::to_string(&DataBase::get_user_skeleton_list()).unwrap()
}


#[get("/raiting/<of>")]
fn get_raiting(of: String) -> String {
    serde_json::to_string(&DataBase::get_raiting(&of)).unwrap()
}

#[get("/get_raiting_skeleton")]
fn get_raiting_skeleton() -> String {
    serde_json::to_string(&DataBase::get_raiting_skeletons()).unwrap()
}

#[get("/get_account/<id>")]
fn get_account(id: String) -> String {
    serde_json::to_string(&DataBase::get_accout_skeleton(&id)).unwrap()
}

#[get("/does_exist_pair?<name>&<birthdate>")]
fn does_exist_pair(name: String, birthdate: String) -> String {
    let mut existance: Option<String> = None;

    for account in &DataBase::get_account_list() {
        if account.name == name && account.birthdate == birthdate {
            existance = Some(account.id.clone());
        }
    }

    serde_json::to_string(&existance).unwrap()
}

#[get("/check_login_credentials?<login>&<password>")]
fn check_login_credentials(login: String, password: String) -> String {
    let result;

    result = DataBase::check_login_credentials(login, password);

    serde_json::to_string(&result).unwrap()
}

#[get("/passwd?<login>&<new_password>")]
fn passwd(login: String, new_password: String) -> &'static str {
    let result = DataBase::check_login_credentials(login.clone(), "ef797c8118f02dfb649607dd5d3f8c7623048c9c063d532cc95c5ed7a898a64f".to_string());
    
    if result {
        if let Some(mut account) = DataBase::get_account(&login.clone()) {
            account.password = new_password;
            DataBase::set_account(account)
        }
    }

    ""
}
