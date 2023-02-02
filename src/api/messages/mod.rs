use serde::Deserialize;
use serde_json::Value;

pub mod client_info;
pub mod message;

use crate::api::messages::client_info::ClientInfo;
use crate::api::messages::message::Message;

#[derive(Deserialize, Debug)]
pub struct Messages {
    pub(crate) client_info: Value,
    pub(crate) message: Value,
}

impl Messages {

    pub(crate) fn get_messages(v: serde_json::Value) -> Self {
        serde_json::from_value(v).unwrap()
    }

    pub fn new_message(val: Value) {
        let messages = Messages::get_messages(val);
        let text = Message::get_message(messages.message).text;

        println!("{:?}", text);
    }
}