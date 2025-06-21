use std::sync::Arc;
use serde_json::Value;
use crate::db_manager::DatabaseManager;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, 
    model::*, schemars,
    service::RequestContext, tool,
};

#[derive(Clone)]
pub struct RbdcDatabaseHandler {
    db_manager: Arc<DatabaseManager>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SqlQueryParams {
    /// 要执行的SQL查询语句
    sql: String,
    /// SQL参数数组，可选
    #[serde(default)]
    params: Vec<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SqlExecParams {
    /// 要执行的SQL修改语句
    sql: String,
    /// SQL参数数组，可选
    #[serde(default)]
    params: Vec<serde_json::Value>,
}

#[tool(tool_box)]
impl RbdcDatabaseHandler {
    pub fn new(db_manager: Arc<DatabaseManager>) -> Self {
        Self { db_manager }
    }

    fn convert_params(&self, params: &[Value]) -> Vec<rbs::Value> {
        params.iter()
            .map(|v| serde_json::from_value(v.clone()).unwrap_or_default())
            .collect()
    }

    #[tool(description = "执行SQL查询并返回结果")]
    async fn sql_query(&self, #[tool(aggr)] SqlQueryParams { sql, params }: SqlQueryParams) -> Result<CallToolResult, McpError> {
        // 转换参数类型从serde_json::Value到rbs::Value
        let rbs_params = self.convert_params(&params);
        
        match self.db_manager.execute_query(&sql, rbs_params).await {
            Ok(results) => {
                let json_str = serde_json::to_string_pretty(&results)
                    .map_err(|e| McpError::internal_error(format!("序列化结果失败: {}", e), None))?;
                Ok(CallToolResult::success(vec![Content::text(json_str)]))
            }
            Err(e) => Err(McpError::internal_error(format!("SQL查询失败: {}", e), None))
        }
    }

    #[tool(description = "执行SQL修改语句（INSERT/UPDATE/DELETE）")]
    async fn sql_exec(&self, #[tool(aggr)] SqlExecParams { sql, params }: SqlExecParams) -> Result<CallToolResult, McpError> {
        // 转换参数类型从serde_json::Value到rbs::Value
        let rbs_params = self.convert_params(&params);
        
        match self.db_manager.execute_modification(&sql, rbs_params).await {
            Ok(result) => {
                let result_str = serde_json::to_string_pretty(&result)
                    .map_err(|e| McpError::internal_error(format!("序列化结果失败: {}", e), None))?;
                Ok(CallToolResult::success(vec![Content::text(result_str)]))
            }
            Err(e) => Err(McpError::internal_error(format!("SQL执行失败: {}", e), None))
        }
    }

    #[tool(description = "获取数据库连接池状态信息")]
    async fn db_status(&self) -> Result<CallToolResult, McpError> {
        let status = self.db_manager.get_pool_state().await;
        let json_str = serde_json::to_string_pretty(&status)
            .map_err(|e| McpError::internal_error(format!("序列化状态失败: {}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for RbdcDatabaseHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "RBDC MCP Server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some("RBDC数据库MCP服务器，提供SQL查询、执行和状态检查工具。支持 sql_query（查询）、sql_exec（修改）和 db_status（状态检查）工具。".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
} 