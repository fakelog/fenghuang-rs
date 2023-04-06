use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vkclient::{longpoll::LongPollRequest, VkApi, VkApiWrapper, Compression};
use tokio::runtime;
use super::Api;

pub struct LongPoll {}

impl LongPoll {
    pub fn init(access_token: String, group_id: String) {

        let group_id = group_id.parse::<usize>().expect("Не удалось преобразовать ID группы в usize");

        let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

        println!("Bot started...");

        runtime.block_on(async move {
            let client: VkApi = vkclient::VkApiBuilder::new(access_token)
                .with_compression(Compression::Zstd)
                .into();

            let BotLongPollResponse { key, server, ts } = client
                .send_request_with_wrapper(BotLongPollRequest { group_id })
                .await
                .unwrap();

            client
                .longpoll()
                .subscribe::<_, Value>(LongPollRequest {
                    key,
                    server,
                    ts,
                    wait: 25,
                    additional_params: (),
                })
                .for_each(|r| async move {
                    match r {
                        Ok(val) => Api::new_event(val),
                        Err(err) => Api::new_error(err)
                    }
                }).await;
        });
    }
}

#[derive(Serialize, Debug)]
struct BotLongPollRequest {
    group_id: usize,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct BotLongPollResponse {
    key: String,
    server: String,
    ts: String,
}

impl VkApiWrapper for BotLongPollRequest {
    type Response = BotLongPollResponse;

    fn get_method_name() -> &'static str {
        "groups.getLongPollServer"
    }
}