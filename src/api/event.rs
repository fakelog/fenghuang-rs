use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub(crate) r#type: String,
    pub(crate) event_id: String,
    pub(crate) v: String,
    pub(crate) object: serde_json::Value,
    pub(crate) group_id: usize,
}

impl Event {

    pub fn get_event(v: Value) -> Self {
        serde_json::from_value(v).unwrap()
    }
}

