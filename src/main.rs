pub mod commands;
pub mod executor;
pub mod history;
pub mod database;
pub mod account;
pub mod hasher;

use database::DataBase;
use rocket::*;
use rocket::serde::json::Json;
use commands::*;
use executor::*;

#[launch]
fn rocket() -> _ {
    Executor::execute(
        Commands::SetBalance { amount: 0, of: String::new() }
    );
    rocket::build().mount("/", routes![index, execute, balance_of, get_history, get_account_skeleton_list, check_login_credentials])
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

#[get("/get_balance/<of>")]
fn balance_of(of: String) -> String {
    DataBase::get_balance(&of).unwrap_or_default().to_string()
}

#[get("/get_history/<of>")]
fn get_history(of: String) -> String {
    serde_json::to_string(&DataBase::get_history(&of).unwrap_or_default()).unwrap()
}

#[get("/get_account_skeleton_list")]
fn get_account_skeleton_list() -> String {
    serde_json::to_string(&DataBase::get_user_skeleton_list()).unwrap()
}

#[get("/check_login_credentials?<login>&<password>")]
fn check_login_credentials(login: String, password: String) -> String {
    serde_json::to_string(&DataBase::check_login_credentials(login, password)).unwrap()
}
