use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct ClientInfo {
    pub(crate) button_actions: Value,
    pub(crate) keyboard: bool,
    pub(crate) inline_keyboard: bool,
    pub(crate) carousel: bool,
    pub(crate) lang_id: u16,

}

impl ClientInfo {

    pub fn get_client_info(v: Value) -> Self {
        serde_json::from_value(v).unwrap()
    }
}