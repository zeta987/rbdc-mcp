# RBDC MCP Server

基于官方 [Model Context Protocol (MCP)](https://modelcontextprotocol.io) 规范的 RBDC 数据库服务器。

## 简介

这个MCP服务器为RBDC数据库连接库提供了标准的MCP工具接口，**默认支持四种数据库类型**（SQLite、MySQL、PostgreSQL、MSSQL）。由于官方的 [`rmcp` Rust SDK](https://github.com/modelcontextprotocol/rust-sdk) 还在开发中，我们实现了一个简化但完整的MCP协议版本。

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

### 构建

```bash
# 从项目根目录
cd rbdc-mcp-server
cargo build --release
```

### 运行

```bash
# 基本用法
./target/release/rbdc-mcp-server --database-url "sqlite://./test.db"

# 配置连接池
./target/release/rbdc-mcp-server \
  --database-url "mysql://user:pass@localhost/mydb" \
  --max-connections 20 \
  --timeout 60

# 设置日志级别
./target/release/rbdc-mcp-server \
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

**基础配置（支持四种数据库类型）：**
```json
{
  "mcpServers": {
    "rbdc-mcp": {
      "command": "C:\\path\\to\\rbdc-mcp-server.exe",
      "args": [
        "--database-url", "sqlite://C:\\path\\to\\database.db",
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
      "command": "C:\\path\\to\\rbdc-mcp-server.exe",
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
      "command": "/path/to/rbdc-mcp-server",
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
      "command": "C:\\path\\to\\rbdc-mcp-server.exe", 
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
        "command": "/path/to/rbdc-mcp-server",
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
2. **命令路径**: `rbdc-mcp-server` 可执行文件的完整路径
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

这个服务器实现了标准的MCP协议，使用JSON-RPC 2.0格式通过stdin/stdout通信。

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
    }
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
    "message": "Internal error: 查询执行失败: table 'users' doesn't exist"
  },
  "id": 3
}
```

## 故障排除

### 常见问题

1. **连接数据库失败**
   - 检查数据库URL格式是否正确
   - 确认数据库服务器是否运行（MySQL/PostgreSQL/MSSQL）
   - 验证用户名密码是否正确
   - SQLite需要确保文件路径存在

2. **Claude Desktop 无法连接**
   - 检查可执行文件路径是否正确
   - 确认配置文件格式是否有效
   - 查看 Claude Desktop 的错误日志

3. **SQL 执行错误**
   - 检查SQL语法是否正确
   - 确认表和字段是否存在
   - 验证SQL参数格式是否匹配

### 调试方法

1. **启用调试日志**
```bash
./rbdc-mcp-server --database-url "sqlite://test.db" --log-level debug
```

2. **测试数据库连接**
```bash
# 可以先单独测试数据库连接
./rbdc-mcp-server --database-url "your-db-url" --log-level debug
```

3. **手动测试MCP协议**
```bash
# 启动服务器并手动发送JSON消息
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocol_version":"2024-11-05","client_info":{"name":"test","version":"1.0.0"}}}' | ./rbdc-mcp-server --database-url "sqlite://test.db"
```

## 开发说明

### 特性

- ✅ **多数据库支持**: 默认支持 SQLite、MySQL、PostgreSQL、MSSQL 四种数据库
- ✅ **MCP 协议兼容**: 完整实现 MCP 2024-11-05 协议规范
- ✅ **参数化查询**: 安全的SQL参数处理，防止SQL注入
- ✅ **连接池管理**: 高效的数据库连接复用
- ✅ **错误处理**: 标准的JSON-RPC 2.0错误响应
- ✅ **日志支持**: 可配置的日志级别

### 未来计划

当官方的 `rmcp` SDK 正式发布后，我们计划：

1. 迁移到官方SDK
2. 添加更多高级功能（资源、提示等）
3. 支持更多传输协议（WebSocket、SSE等）
4. 添加更完善的错误处理

### 当前限制

- 只支持stdio传输
- 简化的MCP协议实现
- 基本的错误处理

### 贡献

欢迎提交Issue和Pull Request来改进这个项目。

## 许可证

Apache 2.0 