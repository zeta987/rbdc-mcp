# RBDC MCP Server

基于官方 [Model Context Protocol (MCP)](https://modelcontextprotocol.io) 规范的 RBDC 数据库服务器。

[![GitHub](https://img.shields.io/badge/GitHub-rbatis%2Frbdc--mcp-blue)](https://github.com/rbatis/rbdc-mcp)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-green)](LICENSE)

## 简介

这个MCP服务器为RBDC数据库连接库提供了标准的MCP工具接口，**默认支持四种数据库类型**（SQLite、MySQL、PostgreSQL、MSSQL）。项目使用官方的 [`rmcp` Rust SDK](https://github.com/modelcontextprotocol/rust-sdk) 构建，确保与MCP协议规范的完全兼容性。

**项目地址**: [https://github.com/rbatis/rbdc-mcp](https://github.com/rbatis/rbdc-mcp)

## 技术栈

- **MCP SDK**: 官方 `rmcp` Rust SDK（最新版本）
- **数据库**: RBDC (Rust DataBase Connectivity)
- **传输协议**: Stdio (标准输入输出)
- **协议版本**: MCP 2024-11-05

## 功能特性

### 支持的工具

1. **sql_query** - 执行SQL查询
   - 执行SELECT等查询语句
   - 返回结构化的查询结果
   - 支持参数化查询

2. **sql_exec** - 执行SQL修改
   - 执行INSERT、UPDATE、DELETE等语句
   - 返回影响的行数
   - 支持参数化查询

3. **db_status** - 获取数据库状态
   - 显示数据库类型
   - 显示连接池状态
   - 显示连接统计信息

### 支持的数据库（默认全部支持）

- **SQLite**: `sqlite://path/to/database.db`
- **MySQL**: `mysql://user:password@host:port/database`
- **PostgreSQL**: `postgres://user:password@host:port/database`
- **MSSQL**: `mssql://user:password@host:port/database`

## 安装和运行

### 快速安装 (推荐)

通过 `cargo install` 直接从 GitHub 安装：

```bash
# 安装最新版本
cargo install --git https://github.com/rbatis/rbdc-mcp rbdc-mcp

# 安装到指定位置
cargo install --git https://github.com/rbatis/rbdc-mcp rbdc-mcp --root ~/.local

# 验证安装
rbdc-mcp --help
```

安装完成后，可执行文件将位于 `~/.cargo/bin/rbdc-mcp`（或指定的目录）。

### 依赖要求

- Rust 1.70+ (建议使用最新稳定版)
- Cargo

### 从源码构建

如果你想从源码构建或进行开发：

```bash
# 克隆仓库
git clone https://github.com/rbatis/rbdc-mcp.git
cd rbdc-mcp

# 构建
cargo build --release

# 可执行文件位于 target/release/rbdc-mcp
```

### 运行

```bash
# 快速开始 (使用 cargo install 安装后)
rbdc-mcp --database-url "sqlite://./test.db"

# 或者使用完整路径
~/.cargo/bin/rbdc-mcp --database-url "sqlite://./test.db"

# 配置连接池
rbdc-mcp \
  --database-url "mysql://user:pass@localhost/mydb" \
  --max-connections 20 \
  --timeout 60

# 设置日志级别
rbdc-mcp \
  --database-url "postgres://user:pass@localhost/mydb" \
  --log-level debug
```

### 命令行参数

- `--database-url, -d`: 数据库连接URL（必需）
- `--max-connections`: 最大连接数（默认：10）
- `--timeout`: 连接超时时间（秒，默认：30）
- `--log-level`: 日志级别（默认：info）

## MCP 客户端配置

### Claude Desktop 配置

在 Claude Desktop 中使用这个服务器，需要在配置文件中添加服务器配置：

#### Windows 配置位置
```
%APPDATA%\Claude\claude_desktop_config.json
```

#### macOS 配置位置
```
~/Library/Application Support/Claude/claude_desktop_config.json
```

#### 配置示例

**基础配置（使用 cargo install 安装后）：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "rbdc-mcp",
      "args": [
        "--database-url", "sqlite://./database.db",
        "--log-level", "info"
      ]
    }
  }
}
```

**Windows 完整路径配置：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "C:\\Users\\YourName\\.cargo\\bin\\rbdc-mcp.exe",
      "args": [
        "--database-url", "sqlite://C:\\path\\to\\database.db",
        "--log-level", "info"
      ]
    }
  }
}
```

**macOS/Linux 配置：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "/home/username/.cargo/bin/rbdc-mcp",
      "args": [
        "--database-url", "sqlite://./database.db",
        "--log-level", "info"
      ]
    }
  }
}
```

**MySQL 配置：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "rbdc-mcp",
      "args": [
        "--database-url", "mysql://user:password@localhost:3306/mydb",
        "--max-connections", "20"
      ]
    }
  }
}
```

**PostgreSQL 配置：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "rbdc-mcp",
      "args": [
        "--database-url", "postgres://user:password@localhost:5432/mydb",
        "--timeout", "60"
      ]
    }
  }
}
```

**MSSQL 配置：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "rbdc-mcp", 
      "args": [
        "--database-url", "mssql://user:password@localhost:1433/mydb",
        "--max-connections", "15"
      ]
    }
  }
}
```

### VS Code MCP 扩展配置

如果使用 VS Code 的 MCP 扩展，在用户设置 JSON 中添加：

```json
{
  "mcp": {
    "servers": {
      "rbdc-mcp": {
        "command": "rbdc-mcp",
        "args": [
          "--database-url", "sqlite://./project.db",
          "--log-level", "debug"
        ]
      }
    }
  }
}
```

### 其他 MCP 客户端

对于其他支持 MCP 的客户端，一般需要配置：

1. **服务器名称**: `rbdc-mcp`
2. **命令路径**: `rbdc-mcp`（如果在 PATH 中）或完整路径
3. **参数**: 数据库连接URL和其他选项
4. **传输方式**: stdio（标准输入输出）

## 使用示例

### 创建测试数据库

```bash
# SQLite 示例
sqlite3 test.db <<EOF
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    age INTEGER,
    email TEXT
);

INSERT INTO users (name, age, email) VALUES 
    ('张三', 25, 'zhangsan@example.com'),
    ('李四', 30, 'lisi@example.com'),
    ('王五', 28, 'wangwu@example.com');
EOF
```

### 在 Claude Desktop 中使用

配置完成后，在 Claude Desktop 中可以这样使用：

```
请帮我查询数据库中所有用户的信息
```

Claude 会自动调用 `sql_query` 工具执行查询。

```
请帮我在数据库中添加一个新用户，姓名是"赵六"，年龄是35
```

Claude 会自动调用 `sql_exec` 工具执行插入操作。

## MCP 协议使用

这个服务器使用官方 rmcp SDK 实现标准的MCP协议，通过JSON-RPC 2.0格式在stdin/stdout上通信。

### 初始化

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocol_version": "2024-11-05",
    "client_info": {
      "name": "test-client",
      "version": "1.0.0"
    },
    "capabilities": {}
  }
}
```

### 列出工具

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

### 调用工具

#### SQL查询示例

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "sql_query",
    "arguments": {
      "sql": "SELECT * FROM users WHERE age > ?",
      "params": [18]
    }
  }
}
```

#### SQL修改示例

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "sql_exec",
    "arguments": {
      "sql": "INSERT INTO users (name, age) VALUES (?, ?)",
      "params": ["张三", 25]
    }
  }
}
```

#### 数据库状态示例

```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "tools/call",
  "params": {
    "name": "db_status",
    "arguments": {}
  }
}
```

## 示例响应

### 查询响应

```json
{
  "jsonrpc": "2.0",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "[{\"id\":1,\"name\":\"张三\",\"age\":25},{\"id\":2,\"name\":\"李四\",\"age\":30}]"
      }
    ],
    "is_error": false
  },
  "id": 3
}
```

### 错误响应

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32603,
    "message": "SQL查询失败: table 'users' doesn't exist"
  },
  "id": 3
}
```

## 故障排除

### 常见问题

1. **安装失败**
   - 确保 Rust 版本 >= 1.70
   - 检查网络连接是否正常
   - 可以尝试手动克隆仓库后构建

2. **连接数据库失败**
   - 检查数据库URL格式是否正确
   - 确认数据库服务器是否运行（MySQL/PostgreSQL/MSSQL）
   - 验证用户名密码是否正确
   - SQLite需要确保文件路径存在

3. **Claude Desktop 无法连接**
   - 检查可执行文件路径是否正确
   - 确认配置文件格式是否有效
   - 查看 Claude Desktop 的错误日志
   - 确保 `rbdc-mcp` 在系统 PATH 中或使用完整路径

4. **SQL 执行错误**
   - 检查SQL语法是否正确
   - 确认表和字段是否存在
   - 验证SQL参数格式是否匹配

### 调试方法

1. **启用调试日志**
```bash
rbdc-mcp --database-url "sqlite://test.db" --log-level debug
```

2. **测试数据库连接**
```bash
# 可以先单独测试数据库连接
rbdc-mcp --database-url "your-db-url" --log-level debug
```

3. **手动测试MCP协议**
```bash
# 启动服务器并手动发送JSON消息
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocol_version":"2024-11-05","client_info":{"name":"test","version":"1.0.0"},"capabilities":{}}}' | rbdc-mcp --database-url "sqlite://test.db"
```

4. **检查安装**
```bash
# 查看版本信息
rbdc-mcp --help

# 查看安装位置
which rbdc-mcp  # Linux/macOS
where rbdc-mcp  # Windows
```

## 开发说明

### 架构特性

- ✅ **官方SDK**: 使用官方 rmcp SDK，确保协议兼容性
- ✅ **多数据库支持**: 默认支持 SQLite、MySQL、PostgreSQL、MSSQL 四种数据库
- ✅ **MCP 协议兼容**: 完整实现 MCP 2024-11-05 协议规范
- ✅ **参数化查询**: 安全的SQL参数处理，防止SQL注入
- ✅ **连接池管理**: 高效的数据库连接复用
- ✅ **错误处理**: 标准的JSON-RPC 2.0错误响应
- ✅ **结构化日志**: 基于 tracing 的结构化日志系统

### 技术实现

- **MCP架构**: 基于官方 rmcp SDK 的工具注册和处理机制
- **异步处理**: 全面使用 tokio 异步运行时
- **类型安全**: 使用 schemars 进行 JSON Schema 验证
- **内存安全**: Rust 语言保证内存安全和线程安全

### 扩展性

项目采用模块化设计，易于扩展：

- `db_manager.rs`: 数据库连接和池管理
- `handler.rs`: MCP 工具实现和服务器处理逻辑
- `main.rs`: 应用程序入口和配置

### 未来计划

使用官方SDK后，我们计划：

1. ✅ **已完成**: 迁移到官方 rmcp SDK
2. 🔄 **进行中**: 添加更多高级功能（资源、提示等）
3. 📅 **计划中**: 支持更多传输协议（WebSocket、SSE等）
4. 📅 **计划中**: 添加数据库连接缓存和优化

### 贡献

欢迎提交Issue和Pull Request来改进这个项目。请确保：

- 遵循 Rust 代码风格
- 添加适当的测试
- 更新相关文档

**贡献地址**: [https://github.com/rbatis/rbdc-mcp](https://github.com/rbatis/rbdc-mcp)

## 许可证

Apache 2.0 