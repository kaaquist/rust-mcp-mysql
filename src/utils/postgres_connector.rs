#![allow(dead_code)]
use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::sse::{Event, Sse},
    routing::post,
    Json, Router,
};
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
use tokio_postgres::{Client, NoTls};

use serde::{Deserialize, Serialize};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;


// MCP Tool Schemas (as per protocol)
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListTablesParams {
    /// Optional schema filter (e.g., "public")
    schema: Option<String>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListTablesResponse {
    /// List of table names
    tables: Vec<String>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryParams {
    /// The SQL SELECT query
    query: String,
    /// Max rows to return (default 100)
    max_rows: Option<i64>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryResponse {
    /// Query results as JSON rows
    rows: Vec<serde_json::Value>,
    /// Column names
    columns: Vec<String>,
}

// App State
#[derive(Clone)]
pub struct AppState {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

// MCP Tool Implementations
async fn list_tables(State(state): State<AppState>, Json(params): Json<ListTablesParams>) -> Result<Json<ListTablesResponse>, StatusCode> {
    let schema = params.schema.unwrap_or_else(|| "public".to_string());
    let client = state.pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let query = format!(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type = 'BASE TABLE'",
        schema
    );

    let rows = client
        .query(&query, &[])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|row| row.get::<_, String>(0))
        .collect();

    Ok(Json(ListTablesResponse { tables: rows }))
}

async fn execute_query(State(state): State<AppState>, Json(params): Json<ExecuteQueryParams>) -> Result<Json<ExecuteQueryResponse>, StatusCode> {
    if !params.query.trim_start().to_uppercase().starts_with("SELECT") {
        return Err(StatusCode::BAD_REQUEST);  // Enforce read-only
    }

    let max_rows = params.max_rows.unwrap_or(100);
    let client = state.pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get columns
    let stmt = client.prepare(&params.query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let columns: Vec<String> = stmt.columns().iter().map(|c| c.name().to_string()).collect();

    // Execute with limit
    let limited_query = format!("{} LIMIT $1", params.query);
    let rows = client
        .query(&limited_query, &[&(max_rows as i64)])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            for (i, col) in columns.iter().enumerate() {
                let val: serde_json::Value = row.get(i);
                map.insert(col.clone(), val);
            }
            serde_json::Value::Object(map)
        })
        .collect();

    Ok(Json(ExecuteQueryResponse { rows, columns }))
}