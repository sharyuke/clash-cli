# Clash-CLI Agent 使用指南

## 构建与测试

```bash
cargo build --release    # Release 构建
cargo build              # Debug 构建
cargo run -- import <url> # 直接运行测试订阅导入
```

## 项目结构

```
src/
├── main.rs              # CLI 入口，命令分发
├── lib.rs               # 模块导出
├── proxy/
│   ├── models.rs        # Config、Proxy、ProxyGroup YAML 模型
│   ├── subscription.rs  # SubscriptionManager（获取/解析/存储订阅）
│   ├── node.rs          # NodeManager
│   └── mode.rs          # ProxyMode 枚举
├── commands/mod.rs       # Clap 命令定义
├── config/mod.rs        # 配置管理
└── utils/              # 日志、网络工具
```

## 关键模式

- **CLI 框架**：Clap 4.x，使用 `#[derive(Subcommand)]` 定义子命令
- **HTTP 客户端**：reqwest，带 `blocking` 特性用于同步请求
- **YAML 解析**：serde_yaml 解析 Mihomo 配置格式
- **订阅存储**：`~/.config/clash-cli/subscriptions.json`

## 已实现命令

| 命令 | 状态 |
|------|------|
| `import <url>` | 可用 - 获取并解析订阅 |
| `sub list` | 可用 |
| `sub add <name> <url>` | 可用 |
| `sub update [name]` | 可用 |
| `sub fetch <url>` | 可用 |

## 待实现功能（来自 REQUIREMENTS.md）

- 核心：start/stop/restart/status、mode、tun
- 节点：test、add、remove
- 代理组：create、select
- 规则：list、add、remove
- 配置：get、set、edit、import/export、validate
- 日志：实时日志查看
