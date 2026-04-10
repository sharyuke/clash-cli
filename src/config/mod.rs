use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyNode {
    pub name: String,
    pub server: String,
    pub port: u16,
    #[serde(rename = "type")]
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyGroup {
    pub name: String,
    pub proxies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClashConfig {
    pub port: Option<u16>,
    #[serde(rename = "socks-port")]
    pub socks_port: Option<u16>,
    #[serde(rename = "mixed-port")]
    pub mixed_port: Option<u16>,
    pub proxies: Option<Vec<ProxyNode>>,
    #[serde(rename = "proxy-groups")]
    pub proxy_groups: Option<Vec<ProxyGroup>>,
}

impl Default for ClashConfig {
    fn default() -> Self {
        Self {
            port: Some(7890),
            socks_port: Some(7891),
            mixed_port: Some(7892),
            proxies: None,
            proxy_groups: None,
        }
    }
}
