use sauron::js_sys::TypeError;
use sauron::prelude::*;
use sauron::web_sys::Response;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

#[derive(Debug, Serialize, Deserialize)]
pub enum FetchStatus<T> {
    Idle(T),
    Loading,
    Complete(T),
    Error(Option<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FetchMsg<T> {
    RequestStarted,
    ReceivedData(Result<Data, T>),
    RequestError(TypeError),
}

impl LoadingBlock {
    pub fn new() -> Self {
        LoadingBlock {
        }
    }

    fn fetch(&self, url: String) -> Cmd<Self, FetchMsg> {
        Http::fetch_with_text_response_decoder(
            &url,
            |v: String| {
                let data: Result<T, _> = serde_json::from_str(&v);
                warn!("data: {:#?}", data);
                data.expect("Error deserializing data")
            },
            FetchMsg::ReceivedData,
            FetchMsg::RequestError,
        )
    }
}