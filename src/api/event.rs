use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub(crate) event_id: String,
    pub(crate) group_id: usize,
    pub(crate) object: serde_json::Value,
    pub(crate) r#type: String,
    pub(crate) v: String,
}

impl Event {
    pub fn to_event(v: Value) -> Self {
        serde_json::from_value(v).unwrap()
    }
}

