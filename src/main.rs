use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Server};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

mod graphql;
mod handlers;
mod routes;
mod state;

use crate::state::AppState;

const PUBLIC_ADDR: &str = "0.0.0.0:3000";
const INTERNAL_ADDR: &str = "0.0.0.0:9090";

async fn serve<E>(
    addr: &'static str,
    app: E,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<(), std::io::Error>
where
    E: poem::IntoEndpoint + 'static,
    E::Endpoint: poem::Endpoint + 'static,
{
    let shutdown_signal = async move {
        let _ = shutdown_rx.recv().await;
    };

    info!("Starting server on {}", addr);

    Server::new(TcpListener::bind(addr))
        .run_with_graceful_shutdown(app, shutdown_signal, None)
        .await
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let state = Arc::new(AppState::default());
    let schema = graphql::build_schema(state.clone());

    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let tx_for_signal = shutdown_tx.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl_c");
        info!("Shutdown signal received, stopping servers...");
        let _ = tx_for_signal.send(());
    });

    let public_app = routes::public_routes(state.clone(), schema).with(Tracing);
    let internal_app = routes::internal_routes(state).with(Tracing);

    #[allow(unused_must_use)]
    {
        tokio::try_join!(
            tokio::spawn(serve(PUBLIC_ADDR, public_app, shutdown_tx.subscribe())),
            tokio::spawn(serve(INTERNAL_ADDR, internal_app, shutdown_tx.subscribe())),
        )?;
    }

    info!("All servers stopped.");
    Ok(())
}