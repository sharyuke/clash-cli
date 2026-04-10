# Clash-CLI Agent Instructions

## Build & Test

```bash
cargo build --release    # Release build
cargo build              # Debug build
cargo run -- import <url> # Run directly with subscription URL
```

## Project Structure

```
src/
├── main.rs              # CLI entry, command dispatch
├── lib.rs               # Module exports
├── proxy/
│   ├── models.rs        # Config, Proxy, ProxyGroup YAML models
│   ├── subscription.rs  # SubscriptionManager (fetch/parse/store)
│   ├── node.rs          # NodeManager
│   └── mode.rs          # ProxyMode enum
├── commands/mod.rs       # Clap command definitions
├── config/mod.rs        # Config management
└── utils/              # Logger, network helpers
```

## Key Patterns

- **CLI Framework**: Clap 4.x with `#[derive(Subcommand)]` for commands
- **HTTP Client**: reqwest with `blocking` feature for sync requests
- **YAML Parsing**: serde_yaml for Mihomo config format
- **Subscription Storage**: `~/.config/clash-cli/subscriptions.json`

## Implemented Commands

| Command | Status |
|---------|--------|
| `import <url>` | Working - fetch & parse subscription |
| `sub list` | Working |
| `sub add <name> <url>` | Working |
| `sub update [name]` | Working |
| `sub fetch <url>` | Working |

## Pending (P0-P2 from REQUIREMENTS.md)

- Core: start/stop/restart/status, mode, tun
- Proxy: test, add, remove
- Group: create, select
- Rule: list, add, remove
- Config: get, set, edit, import/export, validate
- Log: real-time log viewer
