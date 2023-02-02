use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub(crate) attachments: Vec<String>,
    pub(crate) conversation_message_id: usize,
    pub(crate) date: usize,
    pub(crate) from_id: usize,
    pub(crate) fwd_messages: Vec<String>,
    pub(crate) id: usize,
    pub(crate) important: bool,
    pub(crate) is_hidden: bool,
    pub(crate) out: usize,
    pub(crate) peer_id: usize,
    pub(crate) random_id: usize,
    pub(crate) text: String,
}

impl Message {

    pub fn get_message(v: Value) -> Self {
        serde_json::from_value(v).unwrap()
    }

}
