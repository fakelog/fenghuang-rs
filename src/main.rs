extern crate dotenv;

mod api;
mod bot;

use dotenv::dotenv;
use std::env;
use crate::api::longpoll::LongPoll;

fn main() {
    // load env var's
    dotenv().ok();

    let access_token = env::var("TOKEN").expect("Токена не найдено");
    let group_id = env::var("GROUP_ID").expect("ID группы не найдено");

    LongPoll::init(access_token, group_id);
}