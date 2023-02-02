mod api;
mod bot;

use crate::api::longpoll::LongPoll;

fn main() {
    let access_token = std::env::var("SERVICE_TOKEN").unwrap();
    let group_id = std::env::var("GROUP_ID").unwrap().parse().unwrap();

    LongPoll::init(access_token, group_id);
}