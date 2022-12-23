use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Messages {
    pub(crate) client_info: Value,
    pub(crate) message: Value,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    attachments: Vec<String>,
    conversation_message_id: usize,
    date: usize,
    from_id: usize,
    fwd_messages: Vec<String>,
    id: usize,
    important: bool,
    is_hidden: bool,
    out: usize,
    peer_id: usize,
    random_id: usize,
    text: String,
}

impl Messages {
    pub fn new_message(val: Value) {
        let messages = to_event_messages(val);
        let message = to_event_message(messages.message);

        let text = message.text;
        println!("{:?}", text);
    }
}

pub fn to_event_messages(val: serde_json::Value) -> Messages {
    serde_json::from_value(val).unwrap()
}

pub fn to_event_message(val: serde_json::Value) -> Message {
    serde_json::from_value(val).unwrap()
}