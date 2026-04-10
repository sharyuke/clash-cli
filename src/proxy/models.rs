use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "mixed-port")]
    pub mixed_port: Option<u16>,
    #[serde(rename = "port")]
    pub port: Option<u16>,
    #[serde(rename = "socks-port")]
    pub socks_port: Option<u16>,
    #[serde(rename = "redir-port")]
    pub redir_port: Option<u16>,
    #[serde(rename = "tproxy-port")]
    pub tproxy_port: Option<u16>,
    #[serde(rename = "allow-lan")]
    pub allow_lan: Option<bool>,
    #[serde(rename = "bind-address")]
    pub bind_address: Option<String>,
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    #[serde(rename = "log-level")]
    pub log_level: Option<String>,
    #[serde(rename = "external-controller")]
    pub external_controller: Option<String>,
    #[serde(rename = "external-controller-tls")]
    pub external_controller_tls: Option<String>,
    #[serde(rename = "secret")]
    pub secret: Option<String>,
    #[serde(rename = "unified-delay")]
    pub unified_delay: Option<bool>,
    #[serde(rename = "tcp-concurrent")]
    pub tcp_concurrent: Option<bool>,
    #[serde(rename = "dns")]
    pub dns: Option<DnsConfig>,
    #[serde(rename = "proxies")]
    pub proxies: Option<Vec<Proxy>>,
    #[serde(rename = "proxy-groups")]
    pub proxy_groups: Option<Vec<ProxyGroup>>,
    #[serde(rename = "rules")]
    pub rules: Option<Vec<String>>,
    #[serde(rename = "tun")]
    pub tun: Option<TunConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    #[serde(rename = "enable")]
    pub enable: Option<bool>,
    #[serde(rename = "ipv6")]
    pub ipv6: Option<bool>,
    #[serde(rename = "enhanced-mode")]
    pub enhanced_mode: Option<String>,
    #[serde(rename = "fake-ip-range")]
    pub fake_ip_range: Option<String>,
    #[serde(rename = "use-hosts")]
    pub use_hosts: Option<bool>,
    #[serde(rename = "nameserver")]
    pub nameserver: Option<Vec<String>>,
    #[serde(rename = "fallback")]
    pub fallback: Option<Vec<String>>,
    #[serde(rename = "default-nameserver")]
    pub default_nameserver: Option<Vec<String>>,
    #[serde(rename = "nameserver-policy")]
    pub nameserver_policy: Option<HashMap<String, String>>,
    #[serde(rename = "fake-ip-filter")]
    pub fake_ip_filter: Option<Vec<String>>,
    #[serde(rename = "fallback-filter")]
    pub fallback_filter: Option<FallbackFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackFilter {
    #[serde(rename = "geoip")]
    pub geoip: Option<bool>,
    #[serde(rename = "geoip-code")]
    pub geoip_code: Option<String>,
    #[serde(rename = "ipcidr")]
    pub ipcidr: Option<Vec<String>>,
    #[serde(rename = "domain")]
    pub domain: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunConfig {
    #[serde(rename = "enable")]
    pub enable: Option<bool>,
    #[serde(rename = "stack")]
    pub stack: Option<String>,
    #[serde(rename = "auto-route")]
    pub auto_route: Option<bool>,
    #[serde(rename = "auto-detect-interface")]
    pub auto_detect_interface: Option<bool>,
    #[serde(rename = "dns-hijack")]
    pub dns_hijack: Option<Vec<String>>,
    #[serde(rename = "device")]
    pub device: Option<String>,
    #[serde(rename = "mtu")]
    pub mtu: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    #[serde(rename = "server")]
    pub server: String,
    #[serde(rename = "port")]
    pub port: u16,
    #[serde(rename = "cipher")]
    pub cipher: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(rename = "alterId")]
    pub alter_id: Option<u32>,
    #[serde(rename = "udp")]
    pub udp: Option<bool>,
    #[serde(rename = "tls")]
    pub tls: Option<bool>,
    #[serde(rename = "sni")]
    pub sni: Option<String>,
    #[serde(rename = "skip-cert-verify")]
    pub skip_cert_verify: Option<bool>,
    #[serde(rename = "network")]
    pub network: Option<String>,
    #[serde(rename = "flow")]
    pub flow: Option<String>,
    #[serde(rename = "encryption")]
    pub encryption: Option<String>,
    #[serde(rename = "reality-opts")]
    pub reality_opts: Option<RealityOpts>,
    #[serde(rename = "client-fingerprint")]
    pub client_fingerprint: Option<String>,
    #[serde(rename = "obfs")]
    pub obfs: Option<String>,
    #[serde(rename = "obfs-host")]
    pub obfs_host: Option<String>,
    #[serde(rename = "username")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityOpts {
    #[serde(rename = "public-key")]
    pub public_key: Option<String>,
    #[serde(rename = "short-id")]
    pub short_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyGroup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    #[serde(rename = "proxies")]
    pub proxies: Option<Vec<String>>,
    #[serde(rename = "use")]
    pub use_nodes: Option<Vec<String>>,
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "interval")]
    pub interval: Option<u64>,
    #[serde(rename = "tolerance")]
    pub tolerance: Option<u64>,
    #[serde(rename = "timeout")]
    pub timeout: Option<u64>,
    #[serde(rename = "lazy")]
    pub lazy: Option<bool>,
    #[serde(rename = "strategy")]
    pub strategy: Option<String>,
    #[serde(rename = "disable-udp")]
    pub disable_udp: Option<bool>,
    #[serde(rename = "interface-name")]
    pub interface_name: Option<String>,
    #[serde(rename = "routing-mark")]
    pub routing_mark: Option<u32>,
    #[serde(rename = "include-all")]
    pub include_all: Option<bool>,
    #[serde(rename = "include-all-proxies")]
    pub include_all_proxies: Option<bool>,
    #[serde(rename = "include-all-providers")]
    pub include_all_providers: Option<bool>,
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    #[serde(rename = "exclude-filter")]
    pub exclude_filter: Option<String>,
    #[serde(rename = "exclude-type")]
    pub exclude_type: Option<String>,
    #[serde(rename = "expected-status")]
    pub expected_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub rule_type: String,
    pub value: String,
    pub policy: String,
    pub options: Option<Vec<String>>,
}

impl Config {
    pub fn parse(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    pub fn get_proxy_count(&self) -> usize {
        self.proxies.as_ref().map(|p| p.len()).unwrap_or(0)
    }

    pub fn get_group_count(&self) -> usize {
        self.proxy_groups.as_ref().map(|g| g.len()).unwrap_or(0)
    }

    pub fn get_rule_count(&self) -> usize {
        self.rules.as_ref().map(|r| r.len()).unwrap_or(0)
    }
}

impl Proxy {
    pub fn get_type(&self) -> &str {
        &self.proxy_type
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl ProxyGroup {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &str {
        &self.group_type
    }

    pub fn get_proxy_count(&self) -> usize {
        self.proxies.as_ref().map(|p| p.len()).unwrap_or(0)
    }
}
