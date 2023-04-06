use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vkclient::{longpoll::LongPollRequest, VkApi, VkApiWrapper, Compression};
use tokio::runtime;

use super::Api;

pub struct LongPoll {}

impl LongPoll {
    pub fn init(access_token: String, group_id: String) {
        let group_id = match group_id.parse::<usize>() {
            Ok(id) => id,
            Err(err) => {
                panic!("Failed to parse group ID: {}", err);
            }
        };

        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        println!("Bot started...");

        runtime.block_on(async move {
            let client: VkApi = vkclient::VkApiBuilder::new(access_token)
                .with_compression(Compression::Zstd)
                .into();

            let long_poll_response = match client
                .send_request_with_wrapper(BotLongPollRequest { group_id })
                .await
            {
                Ok(response) => response,
                Err(err) => {
                    panic!("Failed to get Long Poll server information: {}", err);
                }
            };

            let LongPollResponse { key, server, ts } = long_poll_response;

            client
                .longpoll()
                .subscribe::<_, Value>(LongPollRequest {
                    key,
                    server,
                    ts,
                    wait: 25,
                    additional_params: serde_json::json!({}),
                })
                .for_each(|r| async move {
                    match r {
                        Ok(val) => Api::new_event(val),
                        Err(err) => Api::new_error(err),
                    }
                })
                .await;
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BotLongPollRequest {
    group_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct LongPollResponse {
    key: String,
    server: String,
    ts: String,
}

impl VkApiWrapper for BotLongPollRequest {
    type Response = LongPollResponse;

    fn get_method_name() -> &'static str {
        "groups.getLongPollServer"
    }
}
