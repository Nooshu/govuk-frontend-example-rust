//! Process entry for the example service.

use govuk_frontend_example_rust::baseline::Policy;
use govuk_frontend_example_rust::config::{self, Config};
use govuk_frontend_example_rust::httpx::Assets;
use govuk_frontend_example_rust::session::SessionStore;
use govuk_frontend_example_rust::web::{router, AppState};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    if let Err(err) = run().await {
        tracing::error!(error = %err, "the service could not start");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let cfg = Config::load()?;
    let policy = Arc::new(Policy::load(&cfg.policy_file)?);
    let assets = Assets::load(
        &cfg.stylesheet,
        &cfg.frontend_assets,
        policy,
        cfg.secure_transport,
    )?;
    let addr = config::listen_addr()?;
    let state = AppState {
        config: Arc::new(cfg.clone()),
        assets,
        sessions: SessionStore::new(),
    };
    let app = router(state);

    tracing::info!(
        %addr,
        govuk_frontend = %cfg.frontend_version,
        demos = cfg.demos_enabled,
        "example service started"
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("listening on {addr}: {e}"))?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| format!("serve: {e}"))?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received");
}

#[allow(dead_code)]
fn _addr_type_check(addr: SocketAddr) -> SocketAddr {
    addr
}
