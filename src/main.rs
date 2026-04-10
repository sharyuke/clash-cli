use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod proxy;

use proxy::{SubscriptionManager, Config};

#[derive(Parser, Debug)]
#[command(name = "clash-cli")]
#[command(version = "0.1.0")]
#[command(about = "A modern CLI proxy client based on Mihomo", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[arg(short, long)]
    mode: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Sub {
        #[command(subcommand)]
        action: SubAction,
    },
    Import {
        url: String,
    },
}

#[derive(Subcommand, Debug)]
enum SubAction {
    List,
    Add {
        name: String,
        url: String,
    },
    Update {
        name: Option<String>,
        #[arg(short, long)]
        url: Option<String>,
    },
    Fetch {
        url: String,
    },
    Delete {
        name: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Sub { action }) => match action {
            SubAction::List => cmd_sub_list()?,
            SubAction::Add { name, url } => cmd_sub_add(&name, &url)?,
            SubAction::Update { name, url } => cmd_sub_update(name.as_deref(), url.as_deref())?,
            SubAction::Fetch { url } => cmd_sub_fetch(&url)?,
            SubAction::Delete { name } => cmd_sub_delete(&name)?,
        },
        Some(Commands::Import { url }) => cmd_import(&url)?,
        None => {
            println!("clash-cli v0.1.0");
            println!("A modern CLI proxy client based on Mihomo");
            println!("\nUse --help for more information");
        }
    }

    Ok(())
}

fn cmd_sub_list() -> Result<()> {
    let manager = SubscriptionManager::new()?;
    let subscriptions = manager.list_subscriptions()?;

    if subscriptions.is_empty() {
        println!("No subscriptions found.");
        return Ok(());
    }

    println!("{}", "=".repeat(80));
    println!("{:<20} {:<50} {}", "NAME", "URL", "UA");
    println!("{}", "=".repeat(80));

    for sub in &subscriptions {
        let ua = sub.ua.clone().unwrap_or_else(|| "clash.meta/v1.19.0".to_string());
        println!("{:<20} {:<50} {}", sub.name, sub.url, ua);
    }

    println!("{}", "=".repeat(80));
    println!("Total: {} subscription(s)", subscriptions.len());

    Ok(())
}

fn cmd_sub_add(name: &str, url: &str) -> Result<()> {
    let manager = SubscriptionManager::new()?;
    let sub = manager.add_subscription(name, url, None)?;
    println!("Added subscription: {}", sub.name);
    Ok(())
}

fn cmd_sub_update(name: Option<&str>, url: Option<&str>) -> Result<()> {
    let manager = SubscriptionManager::new()?;

    if let Some(name) = name {
        let config = manager.update_subscription(name)?;
        print_config_summary(&config);
    } else if let Some(url) = url {
        let config = manager.fetch_subscription(url, None)?;
        print_config_summary(&config);
    } else {
        println!("Please specify --name or --url");
    }

    Ok(())
}

fn cmd_sub_fetch(url: &str) -> Result<()> {
    let manager = SubscriptionManager::new()?;
    let config = manager.fetch_subscription(url, None)?;
    print_config_summary(&config);
    Ok(())
}

fn cmd_sub_delete(name: &str) -> Result<()> {
    let manager = SubscriptionManager::new()?;
    manager.remove_subscription(name)?;
    println!("Deleted subscription: {}", name);
    Ok(())
}

fn cmd_import(url: &str) -> Result<()> {
    let manager = SubscriptionManager::new()?;
    let config = manager.fetch_subscription(url, None)?;
    print_config_summary(&config);
    Ok(())
}

fn print_config_summary(config: &Config) {
    println!("{}", "=".repeat(80));
    println!("Configuration Summary");
    println!("{}", "=".repeat(80));

    println!("\n[General]");
    if let Some(port) = config.port {
        println!("  Port: {}", port);
    }
    if let Some(mixed_port) = config.mixed_port {
        println!("  Mixed Port: {}", mixed_port);
    }
    if let Some(socks_port) = config.socks_port {
        println!("  SOCKS Port: {}", socks_port);
    }
    if let Some(mode) = &config.mode {
        println!("  Mode: {}", mode);
    }
    if let Some(level) = &config.log_level {
        println!("  Log Level: {}", level);
    }
    if let Some(controller) = &config.external_controller {
        println!("  External Controller: {}", controller);
    }

    let proxy_count = config.get_proxy_count();
    let group_count = config.get_group_count();
    let rule_count = config.get_rule_count();

    println!("\n[Proxies]");
    println!("  Count: {}", proxy_count);

    if let Some(proxies) = &config.proxies {
        let mut type_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for p in proxies {
            *type_counts.entry(p.proxy_type.clone()).or_insert(0) += 1;
        }
        for (t, c) in type_counts {
            println!("    {}: {}", t, c);
        }
    }

    println!("\n[Proxy Groups]");
    println!("  Count: {}", group_count);

    if let Some(groups) = &config.proxy_groups {
        for g in groups {
            let count = g.proxies.as_ref().map(|p| p.len()).unwrap_or(0);
            println!("  - {} ({}) [{} proxies]", g.name, g.group_type, count);
        }
    }

    println!("\n[Rules]");
    println!("  Count: {}", rule_count);

    if let Some(rules) = &config.rules {
        let mut type_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for r in rules {
            let rule_type = r.split(',').next().unwrap_or("UNKNOWN");
            *type_counts.entry(rule_type.to_string()).or_insert(0) += 1;
        }
        for (t, c) in type_counts {
            println!("    {}: {}", t, c);
        }
    }

    println!("\n{}", "=".repeat(80));
}
