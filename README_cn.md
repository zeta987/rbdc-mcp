# RBDC MCP Server

基于 [Model Context Protocol (MCP)](https://modelcontextprotocol.io) 的数据库服务器，支持 SQLite、MySQL、PostgreSQL、MSSQL 四种数据库。

**🇺🇸 English Documentation**: [README.md](./README.md)

**🇨🇳 中文文档 / Chinese Documentation**: [README_cn.md](./README_cn.md)

## 优势

- **多数据库支持**: 通过统一接口无缝使用 SQLite、MySQL、PostgreSQL 和 MSSQL
- **AI 集成**: 通过模型上下文协议 (MCP) 与 Claude AI 原生集成
- **零配置**: 自动管理数据库连接和资源
- **安全性**: 通过 AI 驱动的自然语言查询控制对数据库的访问
- **简单易用**: 使用自然语言查询和修改数据库，无需编写 SQL

## 安装

### 🚀 方式一：下载预编译二进制文件（推荐）

从 [GitHub Releases](https://github.com/rbatis/rbdc-mcp/releases) 下载适合你平台的最新版本：

| 平台 | 下载文件 |
|------|----------|
| **Windows (x64)** | `rbdc-mcp-windows-x86_64.exe` |
| **macOS (Intel)** | `rbdc-mcp-macos-x86_64` |
| **macOS (Apple Silicon)** | `rbdc-mcp-macos-aarch64` |
| **Linux (x64)** | `rbdc-mcp-linux-x86_64` |

**安装步骤：**
1. 下载适合你平台的二进制文件
2. **Unix/macOS**: 添加执行权限：`chmod +x rbdc-mcp-*`
3. **可选**: 移动到PATH：`mv rbdc-mcp-* /usr/local/bin/rbdc-mcp`
4. 测试：`./rbdc-mcp --help`

### 🛠️ 方式二：通过 Cargo 安装

**前置要求：** 先安装 [Rust](https://rustup.rs/)。

```bash
cargo install --git https://github.com/rbatis/rbdc-mcp.git
```

### 🔧 方式三：从源码构建

```bash
git clone https://github.com/rbatis/rbdc-mcp.git
cd rbdc-mcp
cargo build --release
# 可执行文件：target/release/rbdc-mcp
```

## 🔧 快速设置

### 步骤 1：配置 Claude Desktop

**配置文件位置：**
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`

**基础配置：**

```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "rbdc-mcp",
      "args": ["--database-url", "sqlite://./database.db"]
    }
  }
}
```

**不同平台配置示例：**

<details>
<summary><strong>Windows（下载的二进制文件）</strong></summary>

```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "C:\\path\\to\\rbdc-mcp-windows-x86_64.exe",
      "args": ["--database-url", "sqlite://C:\\path\\to\\database.db"]
    }
  }
}
```
</details>

<details>
<summary><strong>macOS/Linux（下载的二进制文件）</strong></summary>

```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "/usr/local/bin/rbdc-mcp",
      "args": ["--database-url", "sqlite:///path/to/database.db"]
    }
  }
}
```
</details>

<details>
<summary><strong>Cargo 安装</strong></summary>

```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "rbdc-mcp",
      "args": ["--database-url", "sqlite://./database.db"]
    }
  }
}
```
</details>

### 步骤 2：重启 Claude Desktop

保存配置后，重启 Claude Desktop 以加载 MCP 服务器。

### 步骤 3：测试连接

在 Claude Desktop 中尝试询问：
- "显示数据库连接状态"
- "我的数据库中有哪些表？"

## 📊 使用示例

### 自然语言数据库操作

- **查询数据**: "显示数据库中的所有用户"
- **修改数据**: "添加一个名为张三、邮箱为zhangsan@example.com的新用户"
- **获取状态**: "数据库连接状态如何？"
- **架构信息**: "我的数据库中有哪些表？"

## 🗄️ 数据库支持

| 数据库 | 连接URL格式 |
|--------|-------------|
| **SQLite** | `sqlite://path/to/database.db` |
| **MySQL** | `mysql://user:password@host:port/database` |
| **PostgreSQL** | `postgres://user:password@host:port/database` |
| **MSSQL** | `mssql://user:password@host:port/database` |

## ⚙️ 配置选项

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `--database-url, -d` | 数据库连接URL | *必需* |
| `--max-connections` | 最大连接池大小 | `1` |
| `--timeout` | 连接超时时间（秒） | `30` |
| `--log-level` | 日志级别（error/warn/info/debug） | `info` |

## 🛠️ 可用工具

- **`sql_query`**: 安全执行SELECT查询
- **`sql_exec`**: 执行INSERT/UPDATE/DELETE操作
- **`db_status`**: 检查连接池状态

## 📸 截图

**步骤 1: 配置**
![配置](./step1.png)

**步骤 2: 在Claude中使用**
![使用](./step2.png)

## 许可证

Apache-2.0 