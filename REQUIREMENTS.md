# Clash-CLI 需求规格文档

## 目录

1. [概述](#1-概述)
2. [命令格式规范](#2-命令格式规范)
3. [命令详细规格](#3-命令详细规格)
4. [代理组类型说明](#4-代理组类型说明)
5. [配置文件结构](#5-配置文件结构)
6. [输出格式](#6-输出格式)
7. [退出码](#7-退出码)
8. [环境变量](#8-环境变量)
9. [实现优先级](#9-实现优先级)
10. [节点类型详细参数](#10-节点类型详细参数)
11. [技术架构](#11-技术架构)
12. [Mihomo 内核集成](#12-mihomo-内核集成)
13. [错误处理](#13-错误处理)
14. [设计原则](#14-设计原则)
15. [交互式操作](#15-交互式操作)
16. [帮助系统](#16-帮助系统)
17. [Shell 自动补全](#17-shell-自动补全)
18. [使用示例](#18-使用示例)
19. [未来扩展](#19-未来扩展)

---

## 1. 概述

Clash-CLI 是一个命令行界面的 Clash Meta 代理管理工具，提供节点管理、规则配置、订阅更新等功能。

### 1.1 设计目标

- 简洁高效的命令行交互
- 支持完整的功能操作
- 跨平台支持（Linux/macOS/Windows）

---

## 2. 命令格式规范

### 2.1 基本结构

```
clash [全局选项] <命令> [子命令] [参数] [--标志]
```

### 2.2 命令分类总览

| 命令 | 说明 |
|------|------|
| `clash start` | 启动代理服务 |
| `clash stop` | 停止代理服务 |
| `clash restart` | 重启代理服务 |
| `clash status` | 查看运行状态 |
| `clash mode` | 切换路由模式 |
| `clash tun` | Tun 模式控制 |
| `clash proxy` | 节点管理 |
| `clash group` | 代理组管理 |
| `clash rule` | 规则管理 |
| `clash config` | 配置管理 |
| `clash sub` | 订阅管理 |
| `clash log` | 日志查看 |

---

## 3. 命令详细规格

### 3.1 全局选项

| 选项 | 说明 |
|------|------|
| `-c, --config <path>` | 指定配置文件路径 |
| `-d, --dir <path>` | 指定工作目录 |
| `-o, --output <format>` | 输出格式：table/json/yaml |
| `-q, --quiet` | 静默模式 |
| `-v, --verbose` | 详细输出 |
| `-h, --help` | 显示帮助 |
| `--version` | 显示版本 |

---

### 3.2 核心控制命令

#### 3.2.1 `clash start`

启动 Clash 代理服务。

```
clash start [-p <port>] [--daemon]
```

| 参数/标志 | 说明 |
|----------|------|
| `-p, --port <port>` | 指定 HTTP 代理端口（默认 7890） |
| `--daemon` | 后台守护进程模式 |
| `--tun` | 启动时同时开启 Tun 模式 |

#### 3.2.2 `clash stop`

停止 Clash 代理服务。

```
clash stop [--force]
```

| 参数/标志 | 说明 |
|----------|------|
| `-f, --force` | 强制停止 |

#### 3.2.3 `clash restart`

重启 Clash 代理服务。

```
clash restart [--reload]
```

| 参数/标志 | 说明 |
|----------|------|
| `--reload` | 热更新配置（不中断连接） |

#### 3.2.4 `clash status`

查看当前运行状态。

```
clash status [--json]
```

| 参数/标志 | 说明 |
|----------|------|
| `--json` | JSON 格式输出 |

**输出示例**：
```
● Clash-CLI - 运行中
  模式: Rule
  Tun: 关闭
  HTTP: 127.0.0.1:7890
  SOCKS: 127.0.0.1:7891
  代理组: 自动选择
  节点: HK-01
```

---

### 3.3 模式切换命令

#### 3.3.1 `clash mode`

切换路由模式。

```
clash mode [rule|global|direct] [--show]
```

| 参数/标志 | 说明 |
|----------|------|
| `rule` | 规则模式（根据规则分流） |
| `global` | 全局模式（所有流量走代理） |
| `direct` | 直连模式（不代理） |
| `--show` | 显示当前模式 |

**示例**：
```bash
clash mode rule          # 切换到规则模式
clash mode global        # 切换到全局模式
clash mode direct        # 切换到直连模式
clash mode --show        # 显示当前模式
```

---

### 3.4 Tun 模式命令

#### 3.4.1 `clash tun`

Tun 模式控制。

```
clash tun [on|off|status] [--stack <gvisor|system|winguest>]
```

| 参数/标志 | 说明 |
|----------|------|
| `on` | 开启 Tun 模式 |
| `off` | 关闭 Tun 模式 |
| `status` | 查看 Tun 状态 |
| `--stack <name>` | 指定 TUN 堆栈：gvisor/system/winguest |

**示例**：
```bash
clash tun on             # 开启 Tun 模式
clash tun off            # 关闭 Tun 模式
clash tun status         # 查看 Tun 状态
clash tun on --stack gvisor  # 使用 GVisor 堆栈
```

---

### 3.5 节点管理命令

#### 3.5.1 `clash proxy list`

列出所有节点。

```
clash proxy list [-g <group>] [--format table|json|yaml]
```

| 参数/标志 | 说明 |
|----------|------|
| `-g, --group <name>` | 仅显示指定代理组的节点 |
| `-f, --format <fmt>` | 输出格式 |

**输出示例**：
```
NAME         TYPE      GROUP     DELAY
HK-01        ss        亚太      120ms
HK-02        vmess     亚太      150ms
US-01        trojan    美区      80ms
JP-01        http      日本      200ms
```

#### 3.5.2 `clash proxy select`

选择默认代理节点。

```
clash proxy select <name>
```

| 参数 | 说明 |
|------|------|
| `<name>` | 节点名称 |

**示例**：
```bash
clash proxy select HK-01
```

#### 3.5.3 `clash proxy test`

测试节点延迟。

```
clash proxy test [-g <group>] [--url <url>] [--timeout <sec>]
```

| 参数/标志 | 说明 |
|----------|------|
| `-g, --group <name>` | 测试指定代理组的节点 |
| `-u, --url <url>` | 测试 URL（默认 http://www.gstatic.com） |
| `-t, --timeout <sec>` | 超时时间（默认 5） |

**输出示例**：
```
NAME    TYPE    DELAY    STATUS
HK-01   ss      120ms    ✓
HK-02   vmess   150ms    ✓
US-01   trojan  timeout  ✗
```

#### 3.5.4 `clash proxy add`

添加节点。

```
clash proxy add -n <name> -t <type> -s <server> -p <port> [其他参数]
```

| 参数/标志 | 说明 |
|----------|------|
| `-n, --name <name>` | 节点名称 |
| `-t, --type <type>` | 节点类型：ss/vmess/trojan/http/socks5 |
| `-s, --server <addr>` | 服务器地址 |
| `-p, --port <port>` | 端口 |
| `-u, --username <user>` | 用户名（可选） |
| `-P, --password <pass>` | 密码（可选） |
| `-e, --cipher <cipher>` | 加密方式（ss 必需） |

**示例**：
```bash
clash proxy add -n HK-01 -t ss -s 1.2.3.4 -p 8388 -e aes-256-gcm -P password123
clash proxy add -n US-01 -t vmess -s 5.6.7.8 -p 10086 -u user -P pass
```

#### 3.5.5 `clash proxy remove`

删除节点。

```
clash proxy remove <name>
```

---

### 3.6 代理组管理命令

#### 3.6.1 `clash group list`

列出所有代理组。

```
clash group list [--format table|json|yaml]
```

**输出示例**：
```
NAME          TYPE        CURRENT
🚀 手动选择    select      HK-01
🌍 自动选择    url-test    HK-01
⚖️ 负载均衡    load-balance
🎯 全球直连    select      DIRECT
```

#### 3.6.2 `clash group select`

选择代理组当前节点。

```
clash group select <group> <proxy>
```

**示例**：
```bash
clash group select "🚀 手动选择" HK-01
clash group select "🌍 自动选择" US-01
```

#### 3.6.3 `clash group create`

创建代理组。

```
clash group create -n <name> -t <type> [-i <interval>] [--url <url>]
```

| 参数/标志 | 说明 |
|----------|------|
| `-n, --name <name>` | 代理组名称 |
| `-t, --type <type>` | 类型：select/url-test/fallback/load-balance/relay |
| `-i, --interval <sec>` | 测试间隔（url-test/fallback 用） |
| `-u, --url <url>` | 测试 URL |
| `--benchmark-url <url>` | 基准测试 URL |

**示例**：
```bash
# 创建手动选择代理组
clash group create -n "🚀 手动选择" -t select

# 创建自动测试代理组
clash group create -n "🌍 自动选择" -t url-test -i 300 --url http://www.gstatic.com

# 创建负载均衡代理组
clash group create -n "⚖️ 负载均衡" -t load-balance --strategy round-robin
```

#### 3.6.4 `clash group add`

添加节点到代理组。

```
clash group add <group> <proxy> [--insert]
```

| 参数/标志 | 说明 |
|----------|------|
| `--insert` | 插入到组开头（默认追加到末尾） |

#### 3.6.5 `clash group remove`

从代理组移除节点。

```
clash group remove <group> <proxy>
```

#### 3.6.6 `clash group delete`

删除代理组。

```
clash group delete <name>
```

---

### 3.7 规则管理命令

#### 3.7.1 `clash rule list`

列出所有规则。

```
clash rule list [--type <type>] [--format table|json|yaml]
```

| 参数/标志 | 说明 |
|----------|------|
| `-t, --type <type>` | 规则类型：domain/ip/cidr/port/process |
| `-f, --format <fmt>` | 输出格式 |

**输出示例**：
```
TYPE         VALUE                    POLICY
DOMAIN       google.com               🚀 手动选择
DOMAIN-SFX   apple.com                 DIRECT
DOMAIN-KW    ads                      REJECT
IP-CIDR      192.168.0.0/16           DIRECT
GEOIP        CN                        DIRECT
RULE-SET     proxy                    🚀 手动选择
MATCH        (全部)                    DIRECT
```

#### 3.7.2 `clash rule add`

添加规则。

```
clash rule add <type> <value> <policy> [--comment <note>]
```

| 参数/标志 | 说明 |
|----------|------|
| `<type>` | 规则类型 |
| `<value>` | 规则值 |
| `<policy>` | 目标策略（代理组名/DIRECT/REJECT） |
| `-c, --comment <note>` | 备注说明 |
| `--pre` | 添加到规则列表开头（默认追加） |

**示例**：
```bash
clash rule add DOMAIN google.com "🚀 手动选择"
clash rule add DOMAIN-SUFFIX baidu.com DIRECT
clash rule add DOMAIN-KEYWORD ads REJECT
clash rule add IP-CIDR 10.0.0.0/8 DIRECT
clash rule add GEOIP CN DIRECT
clash rule add RULE-SET proxy "🚀 手动选择"
clash rule add PROCESS-NAME Telegram.exe "📱 Telegram"
clash rule add --pre DOMAIN google.com "🚀 手动选择"  # 插入到最前面
```

**支持的规则类型**：

| 类型 | 说明 | 示例 |
|------|------|------|
| `DOMAIN` | 精确域名 | `DOMAIN google.com` |
| `DOMAIN-SUFFIX` | 域名后缀 | `DOMAIN-SUFFIX google.com` |
| `DOMAIN-KEYWORD` | 域名关键词 | `DOMAIN-KEYWORD google` |
| `GEOSITE` | GeoSite 规则集 | `GEOSITE youtube` |
| `IP-CIDR` | IP 段 | `IP-CIDR 192.168.0.0/16` |
| `IP-CIDR6` | IPv6 段 | `IP-CIDR6 2001:db8::/32` |
| `GEOIP` | GeoIP 国家 | `GEOIP CN` |
| `DST-PORT` | 目标端口 | `DST-PORT 80/443` |
| `PROCESS-NAME` | 进程名 | `PROCESS-NAME Telegram.exe` |
| `RULE-SET` | 远程规则集 | `RULE-SET proxy` |
| `AND` | 逻辑与 | `AND ((DOMAIN-SUFFIX google.com),(GEOIP,!CN))` |
| `OR` | 逻辑或 | `OR ((DOMAIN-KEYWORD google),(DOMAIN-KEYWORD youtube))` |
| `MATCH` | 默认规则 | `MATCH DIRECT` |

#### 3.7.3 `clash rule remove`

删除规则。

```
clash rule remove <index>
```

#### 3.7.4 `clash rule update`

更新远程规则集。

```
clash rule update [--name <name>] [--all]
```

| 参数/标志 | 说明 |
|----------|------|
| `-n, --name <name>` | 更新指定规则集 |
| `-a, --all` | 更新所有规则集 |

#### 3.7.5 `clash rule edit`

编辑规则（交互式）。

```
clash rule edit <index>
```

---

### 3.8 配置管理命令

#### 3.8.1 `clash config get`

获取配置项。

```
clash config get [key]
```

**示例**：
```bash
clash config get                    # 显示所有配置
clash config get port              # 获取端口
clash config get mode              # 获取模式
clash config get dns               # 获取 DNS 配置
```

#### 3.8.2 `clash config set`

设置配置项。

```
clash config set <key> <value>
```

**示例**：
```bash
clash config set port 7890
clash config set mode rule
clash config set log-level info
clash config set allow-lan true
```

#### 3.8.3 `clash config edit`

编辑完整配置（交互式/VIM）。

```
clash config edit [--vim|--nano|--emacs]
```

#### 3.8.4 `clash config export`

导出配置。

```
clash config export [--path <file>] [--format yaml|json]
```

#### 3.8.5 `clash config import`

导入配置。

```
clash config import <file|url> [--merge]
```

| 参数/标志 | 说明 |
|----------|------|
| `--merge` | 合并到现有配置（默认覆盖） |

#### 3.8.6 `clash config validate`

验证配置合法性。

```
clash config validate [--file <path>]
```

---

### 3.9 订阅管理命令

#### 3.9.1 `clash sub list`

列出所有订阅。

```
clash sub list
```

**输出示例**：
```
NAME       URL                                   UPDATED
主订阅      https://example.com/sub/xxx          2024-01-01 12:00
备用订阅    https://backup.com/sub/yyy           2024-01-01 12:00
```

#### 3.9.2 `clash sub add`

添加订阅。

```
clash sub add -n <name> -u <url> [--ua <ua>] [--policy <policy>]
```

| 参数/标志 | 说明 |
|----------|------|
| `-n, --name <name>` | 订阅名称 |
| `-u, --url <url>` | 订阅地址 |
| `--ua <ua>` | 自定义 User-Agent |
| `-p, --policy <policy>` | 默认策略组 |

#### 3.9.3 `clash sub update`

更新订阅。

```
clash sub update [--name <name>|--all] [--merge]
```

| 参数/标志 | 说明 |
|----------|------|
| `-n, --name <name>` | 更新指定订阅 |
| `-a, --all` | 更新所有订阅 |
| `--merge` | 合并到现有配置 |

#### 3.9.4 `clash sub remove`

删除订阅。

```
clash sub remove <name>
```

---

### 3.10 日志命令

#### 3.10.1 `clash log`

查看日志。

```
clash log [--follow] [--level <level>] [--lines <n>]
```

| 参数/标志 | 说明 |
|----------|------|
| `-f, --follow` | 实时跟踪日志 |
| `-l, --level <level>` | 日志级别：debug/info/warn/error |
| `-n, --lines <n>` | 显示行数（默认 100） |

**示例**：
```bash
clash log                     # 显示最近日志
clash log --follow           # 实时跟踪
clash log --level error      # 只显示错误
clash log -n 50              # 显示 50 行
```

---

## 4. 代理组类型说明

| 类型 | 说明 | 特有参数 |
|------|------|----------|
| `select` | 手动选择 | - |
| `url-test` | 自动选择延迟最低节点 | `interval`, `url`, `timeout` |
| `fallback` | 故障转移 | `interval`, `url`, `timeout` |
| `load-balance` | 负载均衡 | `strategy` (round-robin/consistent-hashing) |
| `relay` | 链式代理 | `benchmark-url`, `benchmark-timeout` |

---

## 5. 配置文件结构

### 5.1 默认配置路径

| 平台 | 路径 |
|------|------|
| Linux | `~/.config/clash-cli/config.yaml` |
| macOS | `~/Library/Application Support/clash-cli/config.yaml` |
| Windows | `%APPDATA%\clash-cli\config.yaml` |

### 5.2 配置字段

```yaml
# 基本设置
port: 7890                    # HTTP 代理端口
socks-port: 7891              # SOCKS5 代理端口
tproxy-port: 7893            # TProxy 端口
allow-lan: false              # 允许局域网连接
bind-address: "*"            # 绑定地址
mode: rule                   # 路由模式
log-level: info              # 日志级别

# DNS 配置
dns:
  enable: true
  listen: 0.0.0.0:53
  enhanced-mode: fake-ip
  nameserver:
    - 223.5.5.5
    - 119.29.29.29
  fallback:
    - 8.8.8.8
    - 1.1.1.1

# Tun 配置
tun:
  enable: false
  stack: gvisor
  auto-route: true
  auto-detect-interface: true

# 节点列表
proxies:
  - name: "HK-01"
    type: ss
    server: 1.2.3.4
    port: 8388
    cipher: aes-256-gcm
    password: "password"

# 代理组
proxy-groups:
  - name: "🚀 手动选择"
    type: select
    proxies:
      - HK-01
      - US-01

# 规则
rules:
  - DOMAIN-SUFFIX,google.com,🚀 手动选择
  - GEOIP,CN,DIRECT
  - MATCH,DIRECT
```

---

## 6. 输出格式

### 6.1 Table 格式（默认）

```
NAME         TYPE      STATUS
HK-01        ss        ● 运行中
US-01        vmess     ○ 停止
```

### 6.2 JSON 格式

```json
{
  "proxies": [
    {"name": "HK-01", "type": "ss", "status": "running"}
  ]
}
```

### 6.3 YAML 格式

```yaml
proxies:
  - name: HK-01
    type: ss
    status: running
```

---

## 7. 退出码

| 退出码 | 说明 |
|--------|------|
| 0 | 成功 |
| 1 | 常规错误 |
| 2 | 配置错误 |
| 3 | 网络错误 |
| 4 | 权限错误 |
| 5 | 服务未运行 |
| 6 | 服务已运行 |

---

## 8. 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `CLASH_CONFIG` | 配置文件路径 | 平台默认路径 |
| `CLASH_DIR` | 工作目录 | 平台默认目录 |
| `CLASH_PORT` | HTTP 端口 | 7890 |
| `CLASH_LOG_LEVEL` | 日志级别 | info |

---

## 9. 实现优先级

### P0 - 核心功能
- `start` / `stop` / `restart` / `status`
- `mode`
- `proxy list` / `proxy select`
- `config get` / `config set`

### P1 - 重要功能
- `tun`
- `proxy test`
- `group list` / `group select`
- `rule list` / `rule add` / `rule remove`
- `sub list` / `sub update`

### P2 - 完整功能
- `proxy add` / `proxy remove`
- `group create` / `group add` / `group remove`
- `config edit` / `config import` / `config export`
- `sub add` / `sub remove`
- `log`

---

## 10. 节点类型详细参数

### 10.1 Shadowsocks (ss)

```yaml
proxies:
  - name: "example"
    type: ss
    server: 1.2.3.4
    port: 8388
    cipher: aes-256-gcm
    password: "password"
    # 可选字段
    udp: true
```

| 参数 | 必需 | 说明 |
|------|------|------|
| `server` | 是 | 服务器地址 |
| `port` | 是 | 端口号 |
| `cipher` | 是 | 加密方式：aes-256-gcm/aes-128-gcm/chacha20-ietf-poly1305 |
| `password` | 是 | 密码 |
| `udp` | 否 | 启用 UDP（默认 true） |
| `plugin` | 否 | 插件：obfs/v2ray-plugin |
| `plugin-opts` | 否 | 插件选项 |

### 10.2 VMess

```yaml
proxies:
  - name: "example"
    type: vmess
    server: 1.2.3.4
    port: 10086
    uuid: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
    alterId: 0
    cipher: auto
    # 可选字段
    udp: true
    tls: false
    network: tcp
```

| 参数 | 必需 | 说明 |
|------|------|------|
| `server` | 是 | 服务器地址 |
| `port` | 是 | 端口号 |
| `uuid` | 是 | VMess UUID |
| `alterId` | 是 | 额外 ID（建议为 0） |
| `cipher` | 否 | 加密方式：auto/none/aes-128-gcm/chacha20-poly1305 |
| `network` | 否 | 网络类型：tcp/ws/wss/h2 |
| `tls` | 否 | 启用 TLS |

### 10.3 Trojan

```yaml
proxies:
  - name: "example"
    type: trojan
    server: 1.2.3.4
    port: 443
    password: "password"
    # 可选字段
    udp: true
    sni: example.com
    skip-cert-verify: false
```

| 参数 | 必需 | 说明 |
|------|------|------|
| `server` | 是 | 服务器地址 |
| `port` | 是 | 端口号 |
| `password` | 是 | Trojan 密码 |
| `sni` | 否 | SNI 主机名 |
| `skip-cert-verify` | 否 | 跳过证书验证 |

### 10.4 HTTP/SOCKS5

```yaml
proxies:
  - name: "http-example"
    type: http
    server: 1.2.3.4
    port: 8080
    username: "user"
    password: "pass"
    tls: false

  - name: "socks5-example"
    type: socks5
    server: 1.2.3.4
    port: 1080
    username: "user"
    password: "pass"
    udp: true
```

---

## 11. 技术架构

### 11.1 组件结构

```
┌─────────────────────────────────────────────────────────┐
│                      Clash-CLI                          │
├─────────────────────────────────────────────────────────┤
│  Command Layer (CLI Parser)                             │
│  ├── Global Options                                     │
│  ├── Command Tree                                       │
│  └── Help System                                        │
├─────────────────────────────────────────────────────────┤
│  Core Layer                                             │
│  ├── Config Manager        # 配置读写、合并、校验        │
│  ├── Proxy Manager         # 节点增删改查                │
│  ├── Group Manager         # 代理组管理                 │
│  ├── Rule Manager          # 规则增删改查                 │
│  └── Subscription Manager  # 订阅更新                   │
├─────────────────────────────────────────────────────────┤
│  Service Layer                                         │
│  ├── Mihomo Process Manager # Mihomo 内核进程管理      │
│  ├── API Client            # 与 Mihomo API 通信         │
│  └── TUN Manager           # TUN 模式控制               │
├─────────────────────────────────────────────────────────┤
│  Platform Layer                                        │
│  ├── Linux                  # iptables/nftables         │
│  ├── macOS                  # pfctl                     │
│  └── Windows                # winsock routing           │
└─────────────────────────────────────────────────────────┘
```

### 11.2 Mihomo 内核管理

Clash-CLI 不实现代理功能，而是管理 Mihomo 内核进程：

| 功能 | 说明 |
|------|------|
| **下载内核** | 自动下载/更新 Mihomo 内核二进制 |
| **启动/停止** | 管理 Mihomo 进程生命周期 |
| **配置传递** | 生成配置并通知内核重载 |
| **状态监控** | 通过 API 获取内核运行状态 |

### 11.3 Mihomo API 接口

工具通过 REST API 与 Mihomo 内核通信：

| 接口 | 方法 | 说明 |
|------|------|------|
| `GET /proxies` | GET | 获取所有代理和代理组 |
| `GET /proxies/:name` | GET | 获取指定代理/代理组信息 |
| `PUT /proxies/:name` | PUT | 设置代理组当前节点 |
| `GET /proxies/:name/delay` | GET | 测试指定代理延迟 |
| `GET /rules` | GET | 获取所有规则 |
| `GET /configs` | GET | 获取配置 |
| `PUT /configs` | PUT | 更新配置 |
| `POST /configs?force=true` | POST | 强制重载配置 |
| `GET /traffic` | GET | 获取流量统计 |
| `GET /connections` | GET | 获取连接列表 |
| `DELETE /connections?id=:id` | DELETE | 关闭指定连接 |
| `GET /dns` | GET | DNS 查询 |
| `GET /log` | GET | 获取日志 |

**默认 API 地址**：`http://127.0.0.1:9090`

**认证**：通过 `secret` 字段配置 API 密钥

### 11.4 配置文件热更新

配置更新采用热更新机制，无需重启 Mihomo 内核：

```
1. 修改配置 → 2. 调用 API POST /configs?force=true → 3. Mihomo 平滑更新
```

### 11.5 内核下载与更新

| 平台 | 架构 | 下载 URL 模式 |
|------|------|--------------|
| Linux | amd64 | `https://github.com/MetaCubeX/mihomo/releases/latest/download/mihomo-linux-amd64-v{version}.gz` |
| Linux | arm64 | `https://github.com/MetaCubeX/mihomo/releases/latest/download/mihomo-linux-arm64-v{version}.gz` |
| macOS | amd64 | `https://github.com/MetaCubeX/mihomo/releases/latest/download/mihomo-darwin-amd64-v{version}.gz` |
| macOS | arm64 | `https://github.com/MetaCubeX/mihomo/releases/latest/download/mihomo-darwin-arm64-v{version}.gz` |
| Windows | amd64 | `https://github.com/MetaCubeX/mihomo/releases/latest/download/mihomo-windows-amd64-v{version}.zip` |

**版本检查**：启动时对比本地版本与最新版本，提示更新

---

## 13. 错误处理

### 12.1 错误分类

| 错误类型 | 代码前缀 | 说明 |
|----------|----------|------|
| 配置错误 | `E_CONFIG` | YAML 格式错误、字段缺失 |
| 网络错误 | `E_NETWORK` | 订阅获取失败、API 超时 |
| 权限错误 | `E_PERMISSION` | 端口已被占用、需要 root 权限 |
| 状态错误 | `E_STATE` | 服务未运行、节点不存在 |
| 参数错误 | `E_ARGUMENT` | 无效的参数值 |

### 12.2 错误输出格式

**人类可读格式**（默认）：
```
Error: E_CONFIG_001
  配置文件中缺少必需的字段 'port'
  位置: config.yaml:10
  建议: 添加 'port: 7890' 到配置文件
```

**JSON 格式**（`--json`）：
```json
{
  "error": {
    "code": "E_CONFIG_001",
    "message": "配置文件中缺少必需的字段 'port'",
    "location": "config.yaml:10",
    "hint": "添加 'port: 7890' 到配置文件"
  }
}
```

### 12.3 常见错误码

| 错误码 | 说明 | 解决方案 |
|--------|------|----------|
| `E_CONFIG_001` | 配置文件不存在 | 检查配置文件路径 |
| `E_CONFIG_002` | YAML 解析失败 | 验证 YAML 语法 |
| `E_CONFIG_003` | 缺少必需字段 | 查看文档补全字段 |
| `E_NETWORK_001` | 订阅 URL 无效 | 检查订阅地址 |
| `E_NETWORK_002` | 网络超时 | 检查网络连接 |
| `E_NETWORK_003` | API 请求失败 | 检查 Clash 服务状态 |
| `E_STATE_001` | 服务未运行 | 执行 `clash start` |
| `E_STATE_002` | 服务已运行 | 执行 `clash stop` 或使用 `--force` |
| `E_PERMISSION_001` | 端口已被占用 | 更换端口或停止占用进程 |
| `E_PERMISSION_002` | 需要 root 权限 | 使用 sudo 运行 |

---

## 14. 设计原则

### 13.1 命令行交互原则

1. **一致性**：相同功能的命令具有相似的行为模式
   - `list` 类命令：统一使用 `--format` 输出格式
   - `add` 类命令：统一使用 `-n/--name` 指定名称

2. **可组合性**：命令支持管道和重定向
   ```bash
   clash proxy list --format json | jq '.[] | select(.delay < 100)'
   clash rule list > rules_backup.txt
   ```

3. **幂等性**：重复执行相同命令产生相同结果
   - `clash start` 已运行时报错而非重复启动
   - `clash config set port 7890` 重复执行无副作用

4. **可撤销性**：危险的修改操作提供确认提示
   ```bash
   clash rule remove 1    # 提示确认
   clash config reset     # 提示确认
   ```

### 13.2 配置管理原则

1. **原子性**：配置变更要么完全成功，要么完全回滚
2. **可追溯**：配置变更记录日志，支持回滚
3. **验证优先**：保存前验证配置合法性

### 13.3 性能原则

1. **延迟测试并发**：多个节点延迟测试并行执行
2. **订阅增量更新**：支持增量更新减少带宽
3. **缓存策略**：合理缓存规则集和订阅内容

---

## 15. 交互式操作

### 14.1 选择式交互

当命令参数不完整时，进入交互式选择：

```bash
$ clash proxy select
? 选择节点: (Use arrow keys, Enter to select)
  ● HK-01 (延迟: 120ms)
  ○ HK-02 (延迟: 150ms)
  ○ US-01 (延迟: 80ms)
  ○ JP-01 (延迟: 200ms)
```

### 14.2 确认式交互

执行危险操作时需要确认：

```bash
$ clash stop --force
? 确定要强制停止 Clash 服务吗？ [y/N]
```

### 14.3 多选式交互

批量操作支持多选：

```bash
$ clash rule remove
? 选择要删除的规则: (Press space to select)
  [ ] 1. DOMAIN google.com
  [x] 2. DOMAIN-SUFFIX baidu.com
  [ ] 3. IP-CIDR 192.168.0.0/16
? 已选择 1 条规则，确定删除？ [y/N]
```

---

## 16. 帮助系统

### 15.1 帮助命令格式

```bash
# 全局帮助
clash --help

# 命令帮助
clash help <command>
clash <command> --help

# 子命令帮助
clash help <command> <subcommand>
clash <command> <subcommand> --help
```

### 15.2 帮助信息模板

```
NAME
    clash proxy test - 测试节点延迟

SYNOPSIS
    clash proxy test [-g <group>] [-u <url>] [-t <sec>]

DESCRIPTION
    测试指定节点或代理组的延迟。

    支持并发测试多个节点，结果按延迟排序显示。

OPTIONS
    -g, --group <name>
        测试指定代理组的所有节点

    -u, --url <url>
        测试用 URL（默认 http://www.gstatic.com）

    -t, --timeout <sec>
        超时时间，单位秒（默认 5）

EXAMPLES
    测试所有节点:
        $ clash proxy test

    测试指定代理组:
        $ clash proxy test -g "🚀 手动选择"

    使用自定义 URL 测试:
        $ clash proxy test -u https://www.google.com

SEE ALSO
    clash proxy list, clash proxy select
```

---

## 17. Shell 自动补全

### 16.1 支持的 Shell

| Shell | 补全脚本命令 |
|-------|-------------|
| Bash | `clash completion bash` |
| Zsh | `clash completion zsh` |
| Fish | `clash completion fish` |
| PowerShell | `clash completion powershell` |

### 16.2 安装补全

```bash
# Bash
echo 'source <(clash completion bash)' >> ~/.bashrc

# Zsh
echo 'source <(clash completion zsh)' >> ~/.zshrc

# Fish
clash completion fish > ~/.config/fish/completions/clash.fish
```

### 16.3 补全功能

- 命令名称补全
- 子命令补全
- 参数名称和值补全
- 节点名称补全（从配置读取）
- 代理组名称补全
- 规则类型补全

---

## 18. 使用示例

### 17.1 快速上手

```bash
# 1. 首次启动（使用默认配置）
clash start

# 2. 查看状态
clash status

# 3. 添加订阅
clash sub add -n "主订阅" -u "https://example.com/sub"

# 4. 更新订阅获取节点
clash sub update -n "主订阅"

# 5. 选择节点
clash proxy select HK-01

# 6. 切换到规则模式
clash mode rule
```

### 17.2 高级配置

```bash
# 1. 创建自动选择代理组
clash group create -n "🌍 自动选择" -t url-test -i 300 -u http://www.gstatic.com

# 2. 添加节点到代理组
clash group add "🌍 自动选择" HK-01
clash group add "🌍 自动选择" US-01

# 3. 设置规则
clash rule add DOMAIN-SUFFIX netflix.com "🌍 自动选择"
clash rule add DOMAIN-SUFFIX youtube.com "🌍 自动选择"
clash rule add GEOSITE cn DIRECT

# 4. 开启 Tun 模式
clash tun on --stack gvisor
```

### 17.3 日常维护

```bash
# 查看日志
clash log --level error

# 测试所有节点延迟
clash proxy test

# 更新远程规则集
clash rule update --all

# 导出配置备份
clash config export --path backup.yaml

# 导入配置
clash config import backup.yaml --merge
```

---

## 19. 未来扩展

### P3 - 长期规划

- [ ] 支持代理提供商（Proxy Provider）
- [ ] 支持策略组图标配置
- [ ] Web UI 面板集成
- [ ] 配置热备份到云端
- [ ] 流量统计和报告
- [ ] 定时任务（自动更新订阅）
- [ ] 配置文件版本管理
- [ ] 配置模板市场
- [ ] 节点延迟历史图表
- [ ] 自动选择最优节点
- [ ] 故障告警通知
- [ ] 配置文件对比和差异显示
