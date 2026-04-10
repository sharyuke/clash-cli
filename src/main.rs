use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "clash-cli")]
#[command(version = "0.1.0")]
#[command(about = "A modern CLI proxy client based on Mihomo", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[arg(short, long)]
    mode: Option<String>,

    #[arg(short, long)]
    test: bool,
}

fn main() {
    println!("clash-cli v0.1.0");
    println!("A modern CLI proxy client based on Mihomo");
}
