use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber;

mod handlers;
mod routes;
use routes::create_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_routes();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("🚀 Running NebulaVRF API on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
