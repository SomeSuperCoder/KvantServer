pub mod commands;
pub mod executor;
pub mod history;
pub mod database;
pub mod account;
pub mod hasher;
pub mod time;
pub mod config;

use database::DataBase;
use rocket::*;
use rocket::serde::json::Json;
use commands::*;
use executor::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, execute, get_account_skeleton_list, check_login_credentials, get_account, does_exist_pair])
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

// #[get("/get_balance/<of>")]
// fn balance_of(of: String) -> String {
//     DataBase::get_balance(&of).unwrap_or_default().to_string()
// }

// #[get("/get_history/<of>")]
// fn get_history(of: String) -> String {
//     serde_json::to_string(&DataBase::get_history(&of).unwrap_or_default()).unwrap()
// }

#[get("/get_account_skeleton_list")]
fn get_account_skeleton_list() -> String {
    serde_json::to_string(&DataBase::get_user_skeleton_list()).unwrap()
}

// #[get("/does_exist/<id>")]
// fn does_exist(id: String) -> String {
//     serde_json::to_string(&DataBase::does_exist(&id)).unwrap()
// }

#[get("/get_account/<id>")]
fn get_account(id: String) -> String {
    serde_json::to_string(&DataBase::get_account(&id)).unwrap()
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
