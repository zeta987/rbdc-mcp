//! 数据库连接管理器
//! 
//! 负责管理不同类型的数据库连接

use anyhow::{anyhow, Result};
use rbdc::db::{Connection, Driver};
use rbdc::pool::{ConnectionManager, Pool};
use rbdc_pool_fast::FastPool;
use rbs::Value;
use std::sync::Arc;
use std::time::Duration;

/// 支持的数据库类型
#[derive(Debug, Clone)]
pub enum DatabaseType {
    SQLite,
    MySQL,
    PostgreSQL,
    MSSQL,
}

impl DatabaseType {
    pub fn from_url(url: &str) -> Result<Self> {
        if url.starts_with("sqlite://") {
            Ok(DatabaseType::SQLite)
        } else if url.starts_with("mysql://") {
            Ok(DatabaseType::MySQL)
        } else if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            Ok(DatabaseType::PostgreSQL)
        } else if url.starts_with("mssql://") || url.starts_with("sqlserver://") {
            Ok(DatabaseType::MSSQL)
        } else {
            Err(anyhow!("不支持的数据库URL格式: {}", url))
        }
    }
}

/// 数据库连接管理器
pub struct DatabaseManager {
    pool: Arc<FastPool>,
    db_type: DatabaseType,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub fn new(url: &str) -> Result<Self> {
        let db_type = DatabaseType::from_url(url)?;
        
        let driver: Box<dyn Driver> = match db_type {
            DatabaseType::SQLite => Box::new(rbdc_sqlite::SqliteDriver {}),
            DatabaseType::MySQL => Box::new(rbdc_mysql::MysqlDriver {}),
            DatabaseType::PostgreSQL => Box::new(rbdc_pg::PgDriver {}),
            DatabaseType::MSSQL => Box::new(rbdc_mssql::MssqlDriver {}),
        };

        let manager = ConnectionManager::new(driver, url)?;
        let pool = FastPool::new(manager)?;
        
        Ok(Self {
            pool: Arc::new(pool),
            db_type,
        })
    }

    /// 配置连接池参数
    pub async fn configure_pool(&self, max_connections: u64, timeout_seconds: u64) {
        self.pool.set_max_open_conns(max_connections).await;
        self.pool.set_timeout(Some(Duration::from_secs(timeout_seconds))).await;
    }

    /// 执行查询，返回结果集
    pub async fn execute_query(&self, sql: &str, params: Vec<Value>) -> Result<Vec<Value>> {
        let mut conn = self.pool.get().await
            .map_err(|e| anyhow!("获取数据库连接失败: {}", e))?;
            
        let result = conn.get_values(sql, params).await
            .map_err(|e| anyhow!("执行查询失败: {}", e))?;
            
        Ok(result)
    }

    /// 执行修改操作（INSERT, UPDATE, DELETE等）
    pub async fn execute_modification(&self, sql: &str, params: Vec<Value>) -> Result<serde_json::Value> {
        let mut conn = self.pool.get().await
            .map_err(|e| anyhow!("获取数据库连接失败: {}", e))?;
            
        let result = conn.exec(sql, params).await
            .map_err(|e| anyhow!("执行修改操作失败: {}", e))?;
            
        // 返回操作结果的JSON表示
        Ok(serde_json::json!({
            "rows_affected": result.rows_affected,
            "last_insert_id": result.last_insert_id
        }))
    }

    /// 获取数据库类型
    pub fn database_type(&self) -> &DatabaseType {
        &self.db_type
    }

    /// 获取连接池状态
    pub async fn get_pool_state(&self) -> serde_json::Value {
        let state = self.pool.state().await;
        serde_json::to_value(state).unwrap_or_else(|_| serde_json::json!({}))
    }

    /// 测试数据库连接
    pub async fn test_connection(&self) -> Result<()> {
        let mut conn = self.pool.get().await
            .map_err(|e| anyhow!("获取数据库连接失败: {}", e))?;
            
        conn.ping().await
            .map_err(|e| anyhow!("数据库连接测试失败: {}", e))?;
            
        Ok(())
    }
} 