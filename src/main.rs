use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter};

mod db_manager;
mod handler;

use crate::db_manager::DatabaseManager;
use crate::handler::RbdcDatabaseHandler;
use rmcp::{ServiceExt, transport::stdio};

/// 命令行参数
#[derive(Parser, Debug)]
#[command(name = "rbdc-mcp")]
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
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(args.log_level.parse()
                    .unwrap_or_else(|_| tracing::Level::INFO.into()))
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    info!("启动RBDC MCP服务器");
    info!("数据库URL: {}", args.database_url);

    // 创建数据库管理器
    let db_manager = DatabaseManager::new(&args.database_url)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    
    // 配置连接池
    db_manager.configure_pool(args.max_connections, args.timeout).await;
    
    // 测试数据库连接
    db_manager.test_connection().await
        .map_err(|e| anyhow::Error::msg(format!("数据库连接测试失败: {}", e)))?;
    
    info!("数据库连接测试成功");

    // 创建RBDC数据库处理器
    let handler = RbdcDatabaseHandler::new(Arc::new(db_manager));

    info!("启动RBDC MCP服务器...");
    
    // 启动服务器
    let service = handler.serve(stdio()).await.inspect_err(|e| {
        error!("服务器启动失败: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}