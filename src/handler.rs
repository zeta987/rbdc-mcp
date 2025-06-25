use std::sync::Arc;
use std::future::Future;
use crate::db_manager::DatabaseManager;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, 
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*, schemars,
    service::RequestContext, tool, tool_handler, tool_router,
};

#[derive(Clone)]
pub struct RbdcDatabaseHandler {
    db_manager: Arc<DatabaseManager>,
    tool_router: ToolRouter<RbdcDatabaseHandler>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SqlQueryParams {
    /// SQL query statement to execute
    sql: String,
    /// SQL parameter array, optional
    #[serde(default)]
    params: Vec<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SqlExecParams {
    /// SQL modification statement to execute
    sql: String,
    /// SQL parameter array, optional
    #[serde(default)]
    params: Vec<serde_json::Value>,
}

// Use tool_router macro to generate the tool router
#[tool_router]
impl RbdcDatabaseHandler {
    pub fn new(db_manager: Arc<DatabaseManager>) -> Self {
        Self { 
            db_manager,
            tool_router: Self::tool_router(),
        }
    }

    fn convert_params(&self, params: &[serde_json::Value]) -> Vec<rbs::Value> {
        params.iter()
            .map(|v| serde_json::from_value(v.clone()).unwrap_or_default())
            .collect()
    }

    #[tool(description = "Execute SQL query and return results")]
    async fn sql_query(&self, Parameters(SqlQueryParams { sql, params }): Parameters<SqlQueryParams>) -> Result<CallToolResult, McpError> {
        // Convert parameter types from serde_json::Value to rbs::Value
        let rbs_params = self.convert_params(&params);
        
        match self.db_manager.execute_query(&sql, rbs_params).await {
            Ok(results) => {
                let json_str = serde_json::to_string_pretty(&results)
                    .map_err(|e| McpError::internal_error(format!("Result serialization failed: {}", e), None))?;
                Ok(CallToolResult::success(vec![Content::text(json_str)]))
            }
            Err(e) => Err(McpError::internal_error(format!("SQL query failed: {}", e), None))
        }
    }

    #[tool(description = "Execute SQL modification statements (INSERT/UPDATE/DELETE)")]
    async fn sql_exec(&self, Parameters(SqlExecParams { sql, params }): Parameters<SqlExecParams>) -> Result<CallToolResult, McpError> {
        // Convert parameter types from serde_json::Value to rbs::Value
        let rbs_params = self.convert_params(&params);
        
        match self.db_manager.execute_modification(&sql, rbs_params).await {
            Ok(result) => {
                let result_str = serde_json::to_string_pretty(&result)
                    .map_err(|e| McpError::internal_error(format!("Result serialization failed: {}", e), None))?;
                Ok(CallToolResult::success(vec![Content::text(result_str)]))
            }
            Err(e) => Err(McpError::internal_error(format!("SQL execution failed: {}", e), None))
        }
    }

    #[tool(description = "Get database connection pool status information")]
    async fn db_status(&self) -> Result<CallToolResult, McpError> {
        let status = self.db_manager.get_pool_state().await;
        let json_str = serde_json::to_string_pretty(&status)
            .map_err(|e| McpError::internal_error(format!("Status serialization failed: {}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }
}

#[tool_handler]
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
            instructions: Some("RBDC database MCP server providing SQL query, execution and status check tools. Supports sql_query (query), sql_exec (modification) and db_status (status check) tools.".to_string()),
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