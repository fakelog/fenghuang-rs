use serde_json::Value;
use vkclient::VkApiError;

use crate::api::Api;

pub(crate) struct Bot {}

impl Bot {

    pub fn resources(r: Result<Value, VkApiError>) {
        match r {
            Ok(val) => Api::new_event(val),
            Err(err) => Api::new_error(err)
        }
    }
}