use std::collections::BTreeMap;
use cached::proc_macro::cached;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};
use client::proto::{NetworkInfo, ChainStateUrl};

#[cached(time = 120)]
pub fn get(url: String, auth_header: String) -> BTreeMap<String, NetworkInfo> {
    let nothing: BTreeMap<String, NetworkInfo> = BTreeMap::new();
    let agent: Agent = AgentBuilder::new().timeout_read(Duration::from_secs(5)).build();
    let mut req = agent.get(url.as_str());
    if auth_header.len() > 0 {
        req = req.set("Authorization", auth_header.as_str());
    }
    let response: String = match req.call() {
        Ok(x) => x.into_string().unwrap_or("{}".to_owned()),
        Err(_) => return nothing,
    };
    println!("response {}", response);
    serde_json::from_str(&response).unwrap()
}

pub fn remote_properties(links: Vec<ChainStateUrl>) -> BTreeMap<String, NetworkInfo> {
    let mut res = BTreeMap::new();
    // todo: spawn threads
    for link in links {
        for (k, v) in get(link.url, link.auth_header) {
            println!("remote k={:?} v={:?}", k, v);
            res.insert(k, v);
        }
    }
    res
}