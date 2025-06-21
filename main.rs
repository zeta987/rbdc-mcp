use anyhow::Result;
use clap::Parser;
use log::info;
use std::sync::Arc;

mod db_manager;
mod handler;

use crate::db_manager::DatabaseManager;
use crate::handler::RbdcDatabaseHandler;

use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};

use rust_mcp_sdk::{
    error::SdkResult,
    mcp_server::{server_runtime, ServerRuntime},
    McpServer, StdioTransport, TransportOptions,
};

/// 命令行参数
#[derive(Parser, Debug)]
#[command(name = "rbdc-mcp-server")]
#[command(about = "RBDC MCP服务器 - 提供SQL查询和修改工具")]
struct Args {
    /// 数据库连接URL
    #[arg(short, long)]
    database_url: String,

    /// 最大连接数
    #[arg(long, default_value = "10")]
    max_connections: u64,

    /// 连接超时时间（秒）
    #[arg(long, default_value = "30")]
    timeout: u64,

    /// 日志级别
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // 初始化日志
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, &args.log_level),
    );

    info!("启动RBDC MCP服务器");
    info!("数据库URL: {}", args.database_url);

    // 创建数据库管理器
    let db_manager = DatabaseManager::new(&args.database_url)
        .map_err(|e|anyhow::Error::msg(e.to_string()))?;
    
    // 配置连接池
    db_manager.configure_pool(args.max_connections, args.timeout).await;
    
    // 测试数据库连接
    db_manager.test_connection().await
        .map_err(|e| anyhow::Error::msg(format!("数据库连接测试失败: {}", e)))?;
    
    info!("数据库连接测试成功");

    // STEP 1: 定义服务器详细信息和功能
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "RBDC MCP Server".to_string(),
            version: "1.0.0".to_string(),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some("RBDC数据库MCP服务器，提供SQL查询、执行和状态检查工具".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    // STEP 2: 创建stdio传输
    let transport = StdioTransport::new(TransportOptions::default())
        .map_err(|e| anyhow::Error::msg(format!("创建传输失败: {}", e)))?;

    // STEP 3: 实例化我们的自定义处理器
    let handler = RbdcDatabaseHandler::new(Arc::new(db_manager));

    info!("启动RBDC MCP服务器...");
    
    // STEP 4: 创建MCP服务器
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);

    // STEP 5: 启动服务器
    server.start().await
        .map_err(|e| anyhow::Error::msg(format!("服务器启动失败: {}", e)))
}