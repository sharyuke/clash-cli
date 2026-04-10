use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start {
        #[arg(short, long)]
        config: Option<String>,
    },
    Stop,
    Status,
    Node {
        #[arg(subcommand)]
        action: NodeAction,
    },
    Subscription {
        #[arg(subcommand)]
        action: SubscriptionAction,
    },
    Rule {
        #[arg(subcommand)]
        action: RuleAction,
    },
    Config {
        #[arg(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum NodeAction {
    List,
    Test {
        name: Option<String>,
    },
    Add {
        name: String,
        server: String,
        port: u16,
        #[arg(long, default_value = "http")]
        protocol: String,
    },
    Delete {
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum SubscriptionAction {
    Add {
        name: String,
        url: String,
    },
    Update {
        name: Option<String>,
    },
    List,
    Delete {
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum RuleAction {
    List,
    Add {
        #[arg(short, long)]
        domain: Option<String>,
        #[arg(short, long)]
        ip: Option<String>,
        rule_type: String,
        proxy: String,
    },
    Delete {
        index: usize,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    Show,
    Edit,
    Import {
        path: String,
    },
    Export {
        path: String,
    },
}
