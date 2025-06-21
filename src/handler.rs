use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
    ListToolsResult, RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};
use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    tool_box,
};

use serde_json::Value;
use std::sync::Arc;
use crate::db_manager::DatabaseManager;

//***************//
//  SqlQueryTool //
//***************//
#[mcp_tool(
    name = "sql_query",
    description = "执行SQL查询并返回结果",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct SqlQueryTool {
    /// 要执行的SQL查询语句
    sql: String,
    /// SQL参数数组，可选
    #[serde(default)]
    params: Vec<serde_json::Value>,
}

impl SqlQueryTool {
    pub async fn call_tool(&self, db_manager: &DatabaseManager) -> Result<CallToolResult, CallToolError> {
        // 转换参数类型从serde_json::Value到rbs::Value
        let rbs_params = self.convert_params(&self.params);
        
        match db_manager.execute_query(&self.sql, rbs_params).await {
            Ok(results) => {
                let json_str = serde_json::to_string_pretty(&results)
                    .map_err(|e| CallToolError::new(rbdc::Error::from(format!("序列化结果失败: {}", e))))?;
                Ok(CallToolResult::text_content(json_str, None))
            }
            Err(e) => Err(CallToolError::new(rbdc::Error::from(format!("SQL查询失败: {}", e))))
        }
    }

    fn convert_params(&self, params: &[Value]) -> Vec<rbs::Value> {
        params.iter()
            .map(|v| match v {
                Value::String(s) => rbs::Value::String(s.clone()),
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        rbs::Value::I64(i)
                    } else if let Some(f) = n.as_f64() {
                        rbs::Value::F64(f)
                    } else {
                        rbs::Value::String(n.to_string())
                    }
                },
                Value::Bool(b) => rbs::Value::Bool(*b),
                Value::Null => rbs::Value::Null,
                _ => rbs::Value::String(v.to_string()),
            })
            .collect()
    }
}

//**************//
//  SqlExecTool //
//**************//
#[mcp_tool(
    name = "sql_exec",
    description = "执行SQL修改语句（INSERT/UPDATE/DELETE）",
    idempotent_hint = false,
    destructive_hint = true,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct SqlExecTool {
    /// 要执行的SQL修改语句
    sql: String,
    /// SQL参数数组，可选
    #[serde(default)]
    params: Vec<serde_json::Value>,
}

impl SqlExecTool {
    pub async fn call_tool(&self, db_manager: &DatabaseManager) -> Result<CallToolResult, CallToolError> {
        // 转换参数类型从serde_json::Value到rbs::Value
        let rbs_params = self.convert_params(&self.params);
        
        match db_manager.execute_modification(&self.sql, rbs_params).await {
            Ok(result) => {
                let result_str = serde_json::to_string_pretty(&result)
                    .map_err(|e| CallToolError::new(rbdc::Error::from(format!("序列化结果失败: {}", e))))?;
                Ok(CallToolResult::text_content(result_str, None))
            }
            Err(e) => Err(CallToolError::new(rbdc::Error::from(format!("SQL执行失败: {}", e))))
        }
    }

    fn convert_params(&self, params: &[Value]) -> Vec<rbs::Value> {
        params.iter()
            .map(|v| match v {
                Value::String(s) => rbs::Value::String(s.clone()),
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        rbs::Value::I64(i)
                    } else if let Some(f) = n.as_f64() {
                        rbs::Value::F64(f)
                    } else {
                        rbs::Value::String(n.to_string())
                    }
                },
                Value::Bool(b) => rbs::Value::Bool(*b),
                Value::Null => rbs::Value::Null,
                _ => rbs::Value::String(v.to_string()),
            })
            .collect()
    }
}

//**************//
//  DbStatusTool //
//**************//
#[mcp_tool(
    name = "db_status",
    description = "获取数据库连接池状态信息",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct DbStatusTool {
    // 无参数
}

impl DbStatusTool {
    pub async fn call_tool(&self, db_manager: &DatabaseManager) -> Result<CallToolResult, CallToolError> {
        let status = db_manager.get_pool_state().await;
        let json_str = serde_json::to_string_pretty(&status)
            .map_err(|e| CallToolError::new(rbdc::Error::from(format!("序列化状态失败: {}", e))))?;
        Ok(CallToolResult::text_content(json_str, None))
    }
}

//***************//
//  DatabaseTools //
//***************//
// 生成一个名为DatabaseTools的枚举，包含所有工具变体
tool_box!(DatabaseTools, [SqlQueryTool, SqlExecTool, DbStatusTool]);

//************************//
//  RbdcDatabaseHandler   //
//************************//
pub struct RbdcDatabaseHandler {
    db_manager: Arc<DatabaseManager>,
}

impl RbdcDatabaseHandler {
    pub fn new(db_manager: Arc<DatabaseManager>) -> Self {
        Self { db_manager }
    }
}

#[async_trait]
impl ServerHandler for RbdcDatabaseHandler {
    /// 处理工具列表请求，返回可用工具列表
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: DatabaseTools::tools(),
        })
    }

    /// 处理工具调用请求，并使用相应的工具处理
    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        // 尝试将请求参数转换为DatabaseTools枚举
        let tool_params: DatabaseTools =
            DatabaseTools::try_from(request.params).map_err(CallToolError::new)?;

        // 匹配工具变体并执行相应的逻辑
        match tool_params {
            DatabaseTools::SqlQueryTool(sql_query_tool) => {
                sql_query_tool.call_tool(&self.db_manager).await
            }
            DatabaseTools::SqlExecTool(sql_exec_tool) => {
                sql_exec_tool.call_tool(&self.db_manager).await
            }
            DatabaseTools::DbStatusTool(db_status_tool) => {
                db_status_tool.call_tool(&self.db_manager).await
            }
        }
    }
} 