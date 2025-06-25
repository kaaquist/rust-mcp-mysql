use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};
mod utils;
use utils::counter::Counter;
const BIND_ADDRESS: &str = "127.0.0.1:9955";


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::Registry::default()
        .with(
            // stdout layer, to view everything in the console
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(true)
        )
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        );
    tracing::subscriber::set_global_default(subscriber)?;
    let config = SseServerConfig {
        bind: BIND_ADDRESS.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };

    let (sse_server, router) = SseServer::new(config);

    // Do something with the router, e.g., add routes or middleware

    let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;

    let ct = sse_server.config.ct.child_token();
    
    tracing::info!("sse server started!");
    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        ct.cancelled().await;
        tracing::info!("sse server cancelled");
    });

    tokio::spawn(async move {
        if let Err(e) = server.await {
            tracing::error!(error = %e, "sse server shutdown with error");
        }
    });

    let ct = sse_server.with_service(Counter::new);

    tokio::signal::ctrl_c().await?;
    tracing::info!("sse server stopped!");
    ct.cancel();
    Ok(())
}