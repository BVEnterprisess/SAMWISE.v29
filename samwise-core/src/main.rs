mod circuit_breaker;
mod engine_wrapper;
mod errors;
mod proxy;
mod server;

use circuit_breaker::CircuitBreaker;
use compact_str::CompactString;
use engine_wrapper::EngineWrapper;
use miette::IntoDiagnostic;
use proxy::create_proxy_router;
use server::{AppState, create_router};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "samwise_core=info,tower_http=info".into()),
        )
        .json()
        .init();

    let db_path = std::env::var("YANTRIKDB_PATH").unwrap_or_else(|_| "./yantrikdb.sqlite".into());
    let state_path = std::env::var("STATE_DB_PATH").unwrap_or_else(|_| "./sidecar_state.db".into());
    let llm_api_base = CompactString::from(
        std::env::var("LLM_API_BASE").unwrap_or_else(|_| "http://127.0.0.1:20128/v1".into()),
    );

    let engine = EngineWrapper::new(db_path, state_path).await?;

    let shared_state = Arc::new(AppState {
        engine,
        breaker: CircuitBreaker::new(5, 30),
    });

    let shutdown = CancellationToken::new();

    // 1. Memory Sidecar API Server
    let sidecar_app = create_router(Arc::clone(&shared_state));
    let sidecar_listener = tokio::net::TcpListener::bind("127.0.0.1:30001")
        .await
        .into_diagnostic()?;
    tracing::info!("Memory Sidecar active on 127.0.0.1:30001");

    let sidecar_shutdown = shutdown.clone();
    let sidecar_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(sidecar_listener, sidecar_app)
            .with_graceful_shutdown(async move { sidecar_shutdown.cancelled().await })
            .await
        {
            tracing::error!(error = %e, "Sidecar server failed");
        }
    });

    // 2. MetaProxy Router
    let proxy_app = create_proxy_router(Arc::clone(&shared_state), llm_api_base);
    let proxy_listener = tokio::net::TcpListener::bind("0.0.0.0:30000")
        .await
        .into_diagnostic()?;
    tracing::info!("MetaProxy active on 0.0.0.0:30000");

    let proxy_shutdown = shutdown.clone();
    let proxy_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(proxy_listener, proxy_app)
            .with_graceful_shutdown(async move { proxy_shutdown.cancelled().await })
            .await
        {
            tracing::error!(error = %e, "Proxy server failed");
        }
    });

    wait_for_shutdown_signal().await;
    tracing::info!("Shutdown signal received. Initiating graceful drain...");
    shutdown.cancel();

    let _ = tokio::join!(sidecar_handle, proxy_handle);
    tracing::info!("All servers safely shut down.");

    Ok(())
}

async fn wait_for_shutdown_signal() {
    let ctrl_c = async {
        if let Err(e) = tokio::signal::ctrl_c().await {
            tracing::error!(error = %e, "Failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut sig) => {
                sig.recv().await;
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to install SIGTERM handler");
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Received SIGINT (Ctrl+C)"),
        _ = terminate => tracing::info!("Received SIGTERM"),
    }
}
