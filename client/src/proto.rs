use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkPropValue {
    /// label of the property
    pub label: String,
    /// Value of the property
    pub value: String,
    /// Link to the details of the property
    pub link: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkInfo {
    /// name of the link to the the explorer of this network
    pub link: Option<String>,
    /// name of the blockchain network
    pub name: String,
    /// whether this is a test framework
    pub test: Option<bool>,
    /// name of the coin behind this network
    pub coin: Option<String>,
    /// price of the coin, if this is not a test network
    pub price: Option<f32>,
    /// map of properties of the current state
    /// (number of daily/hourly transactions)
    pub properties: Vec<NetworkPropValue>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainStateUrl {
    /// url where we should get chains state
    pub url: String,
    /// authorization header for the chains state
    pub auth_header: String,
}