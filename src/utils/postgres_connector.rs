#![allow(dead_code)]
use std::sync::Arc;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};

use serde_json::json;
use tokio::sync::Mutex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


// MCP Tool Schemas (as per protocol)
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListTablesParams {
    /// Optional schema filter (e.g., "public")
    schema: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListTablesResponse {
    /// List of table names
    tables: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryParams {
    /// The SQL SELECT query
    query: String,
    /// Max rows to return (default 100)
    max_rows: Option<i64>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryResponse {
    /// Query results as JSON rows
    rows: Vec<serde_json::Value>,
    /// Column names
    columns: Vec<String>,
}