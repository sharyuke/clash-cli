use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use reqwest::blocking::Client;

use crate::proxy::models::{Config, Proxy, ProxyGroup};

const DEFAULT_USER_AGENT: &str = "clash.meta/v1.19.0";
const DEFAULT_CONFIG_DIR: &str = ".config/clash-cli";
const SUBSCRIPTIONS_FILE: &str = "subscriptions.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub name: String,
    pub url: String,
    pub ua: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub custom_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStore {
    pub subscriptions: Vec<Subscription>,
}

pub struct SubscriptionManager {
    client: Client,
    config_dir: PathBuf,
}

impl SubscriptionManager {
    pub fn new() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        Ok(Self {
            client: Client::builder()
                .user_agent(DEFAULT_USER_AGENT)
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .context("Failed to create HTTP client")?,
            config_dir,
        })
    }

    pub fn with_config_dir(config_dir: PathBuf) -> Result<Self> {
        let client = Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { client, config_dir })
    }

    fn get_config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Cannot find home directory")?;
        Ok(home.join(DEFAULT_CONFIG_DIR))
    }

    fn get_store_path(&self) -> PathBuf {
        self.config_dir.join(SUBSCRIPTIONS_FILE)
    }

    pub fn load_subscriptions(&self) -> Result<Vec<Subscription>> {
        let path = self.get_store_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&path)?;
        let store: SubscriptionStore = serde_json::from_str(&content)?;
        Ok(store.subscriptions)
    }

    pub fn save_subscriptions(&self, subscriptions: &[Subscription]) -> Result<()> {
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir)?;
        }
        let store = SubscriptionStore {
            subscriptions: subscriptions.to_vec(),
        };
        let content = serde_json::to_string_pretty(&store)?;
        fs::write(self.get_store_path(), content)?;
        Ok(())
    }

    pub fn add_subscription(&self, name: &str, url: &str, ua: Option<String>) -> Result<Subscription> {
        let mut subscriptions = self.load_subscriptions()?;

        let sub = Subscription {
            name: name.to_string(),
            url: url.to_string(),
            ua,
            updated_at: None,
            custom_fields: HashMap::new(),
        };

        subscriptions.push(sub.clone());
        self.save_subscriptions(&subscriptions)?;

        Ok(sub)
    }

    pub fn remove_subscription(&self, name: &str) -> Result<()> {
        let mut subscriptions = self.load_subscriptions()?;
        subscriptions.retain(|s| s.name != name);
        self.save_subscriptions(&subscriptions)?;
        Ok(())
    }

    pub fn fetch_subscription(&self, url: &str, ua: Option<&str>) -> Result<Config> {
        let mut request = self.client.get(url);
        
        let user_agent = ua.unwrap_or(DEFAULT_USER_AGENT);
        request = request.header("User-Agent", user_agent);

        let response = request.send().context("Failed to fetch subscription")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Subscription fetch failed with status: {}",
                response.status()
            );
        }

        let content = response.text().context("Failed to read subscription content")?;

        let content = Self::decode_content(&content)?;

        let config: Config = serde_yaml::from_str(&content)
            .context("Failed to parse subscription as YAML")?;

        Ok(config)
    }

    fn decode_content(content: &str) -> Result<String> {
        if content.starts_with("dmVj") || content.contains("H4sI") {
            if let Ok(decoded) = Self::decode_gzip(content) {
                return Ok(decoded);
            }
        }
        Ok(content.to_string())
    }

    fn decode_gzip(content: &str) -> Result<String> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let decoded = base64_decode(content);
        let mut gz = GzDecoder::new(&decoded[..]);
        let mut s = String::new();
        gz.read_to_string(&mut s)?;
        Ok(s)
    }

    pub fn fetch_and_parse(&self, url: &str) -> Result<(Config, Vec<Proxy>, Vec<ProxyGroup>)> {
        let config = self.fetch_subscription(url, None)?;

        let proxies = config.proxies.clone().unwrap_or_default();
        let groups = config.proxy_groups.clone().unwrap_or_default();

        Ok((config, proxies, groups))
    }

    pub fn list_subscriptions(&self) -> Result<Vec<Subscription>> {
        self.load_subscriptions()
    }

    pub fn update_subscription(&self, name: &str) -> Result<Config> {
        let subscriptions = self.load_subscriptions()?;
        let sub = subscriptions
            .iter()
            .find(|s| s.name == name)
            .context("Subscription not found")?;

        self.fetch_subscription(&sub.url, sub.ua.as_deref())
    }
}

fn base64_decode(input: &str) -> Vec<u8> {
    let input = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    
    let decoding_table: Vec<i8> = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .iter()
        .map(|&b| b as i8)
        .collect();

    let mut output = Vec::new();
    let mut buffer = 0i32;
    let mut bits_collected = 0;

    for c in input.chars() {
        if c == '=' {
            break;
        }
        
        let value = match decoding_table.iter().position(|&v| v == c as i8) {
            Some(v) => v as i32,
            None => continue,
        };

        buffer = (buffer << 6) | value;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            output.push(((buffer >> bits_collected) & 0xFF) as u8);
        }
    }

    output
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new().expect("Failed to create SubscriptionManager")
    }
}
