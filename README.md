# RBDC MCP Server

A database server based on [Model Context Protocol (MCP)](https://modelcontextprotocol.io), supporting SQLite, MySQL, PostgreSQL, and MSSQL databases.

**🇨🇳 中文文档 / Chinese Documentation**: [README_cn.md](./README_cn.md)

## Advantages

- **Multiple Database Support**: Seamlessly work with SQLite, MySQL, PostgreSQL, and MSSQL using a unified interface
- **AI Integration**: Native integration with Claude AI through the Model Context Protocol
- **Zero Configuration**: Automatic management of database connections and resources
- **Security**: Controlled access to your database through AI-driven natural language queries
- **Simplicity**: Use natural language to query and modify your database without writing SQL

## Installation

### Prerequisites

First, you need to install Rust and Cargo:

**Windows:**
1. Visit [https://rustup.rs/](https://rustup.rs/)
2. Download and run `rustup-init.exe`
3. Restart your terminal to refresh environment variables

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Method 1: Install from Git Repository (Recommended)
```bash
cargo install --git https://github.com/rbatis/rbdc-mcp.git
```

### Method 2: Build from Source
```bash
git clone https://github.com/rbatis/rbdc-mcp.git
cd rbdc-mcp
cargo build --release
# Executable located at target/release/rbdc-mcp
```

### Setup Guide

Step 1: Configure the Claude Desktop settings
![Step 1: Configuration](./step1.png)

Step 2: Use database commands in Claude
![Step 2: Usage](./step2.png)

## Usage

### Start the Server (Manual, Optional)
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

### Configure Claude Desktop

Edit the configuration file:
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

**Windows Full Path Example:**
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

### Available Tools

After configuration, you can use the following features in Claude Desktop:

- **Query Data**: "Help me query all users in the database"
- **Modify Data**: "Add a new user to the database"  
- **Get Status**: "Show database connection status"

### Command Line Arguments

- `--database-url, -d`: Database connection URL (required)
- `--max-connections`: Maximum number of connections (default: 1)
- `--timeout`: Connection timeout in seconds (default: 30)
- `--log-level`: Log level (default: info)

## Supported Databases

- **SQLite**: `sqlite://path/to/database.db`
- **MySQL**: `mysql://user:password@host:port/database`
- **PostgreSQL**: `postgres://user:password@host:port/database`
- **MSSQL**: `mssql://user:password@host:port/database`

## Features

- **sql_query**: Execute SQL queries (SELECT statements)
- **sql_exec**: Execute SQL modifications (INSERT/UPDATE/DELETE)  
- **db_status**: Get database connection pool status

## License

Apache-2.0 