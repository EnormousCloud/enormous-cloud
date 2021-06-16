// use cached::proc_macro::cached;
// use serde::Deserialize;
// use std::collections::HashMap;
// use std::time::Duration;
// use ureq::{Agent, AgentBuilder};
// use client::proto::{NetworkInfo, NetworkPropValue};

// #[cached(time = 120)]
// pub fn get(networks: Vec<NetworkInfo>, url: String) -> Vec<NetworkInfo> {
//     let agent: Agent = AgentBuilder::new()
//         .timeout_read(Duration::from_secs(5))
//         .build();
//     let response: String = match agent.get(url.as_str()).call() {
//         Ok(x) => x.into_string().unwrap_or("{}".to_owned()),
//         Err(_) => return ,
//     };
//     let kv: HashMap<String, NetworkInfo> = match serde_json::from_str(&response) {
//         Ok(x) => x,
//         Err(_) => return vec![],
//     };
//     let mut res: Vec<NetworkInfo> = vec![];
//     res
// }
