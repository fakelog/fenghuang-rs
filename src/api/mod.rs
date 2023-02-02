
use serde_json::Value;
use vkclient::VkApiError;

pub mod event;
pub mod longpoll;
mod messages;

pub use crate::api::event::Event;
pub use crate::api::messages::Messages;

pub struct Api {}

impl Api {

    pub(crate) fn new_event(v: Value) {
        let event = Event::get_event(v);

        if event.r#type == "message_new" {
            Messages::new_message(event.object);
        }

    }

    pub(crate) fn new_error(e: VkApiError) {
        print!("{:?}", e)
    }

}
