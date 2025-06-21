# RBDC MCP Server

基于 [Model Context Protocol (MCP)](https://modelcontextprotocol.io) 的数据库服务器，支持 SQLite、MySQL、PostgreSQL、MSSQL 四种数据库。

## 安装

### 方式一：从 Git 仓库安装（推荐）
```bash
cargo install --git https://github.com/rbatis/rbdc-mcp.git
```

### 方式二：从源码构建
```bash
git clone https://github.com/rbatis/rbdc-mcp.git
cd rbdc-mcp
cargo build --release
# 可执行文件位于 target/release/rbdc-mcp
```

## 使用

### 启动服务器(手动，可不执行)
```bash
# SQLite
rbdc-mcp --database-url "sqlite://./database.db"

# MySQL  
rbdc-mcp --database-url "mysql://user:password@localhost:3306/database"

# PostgreSQL
rbdc-mcp --database-url "postgres://user:password@localhost:5432/database"

# MSSQL
rbdc-mcp --database-url "mssql://user:password@localhost:1433/database"
```

### 配置 Claude Desktop

编辑配置文件：
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "rbdc-mcp",
      "args": [
        "--database-url", "sqlite://./database.db"
      ]
    }
  }
}
```

**Windows 完整路径示例：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "C:\\Users\\YourName\\.cargo\\bin\\rbdc-mcp.exe",
      "args": [
        "--database-url", "sqlite://C:\\path\\to\\database.db"
      ]
    }
  }
}
```

### 可用工具

配置完成后，在 Claude Desktop 中可以使用以下功能：

- **查询数据**: "帮我查询数据库中的所有用户"
- **修改数据**: "在数据库中添加一个新用户"  
- **获取状态**: "显示数据库连接状态"

### 命令行参数

- `--database-url, -d`: 数据库连接URL（必需）
- `--max-connections`: 最大连接数（默认：10）
- `--timeout`: 连接超时时间秒数（默认：30）
- `--log-level`: 日志级别（默认：info）

## 许可证

Apache-2.0 