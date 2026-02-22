use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let ip_addr = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(ip_addr).await.unwrap();
    let api_router = Router::new().nest(
        "/api",
        Router::new()
            .merge(authorization::router())
            .merge(slice_2::router())
            .merge(slice_3::router()),
    );
    let spa = ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html"));
    let router = api_router.fallback_service(spa);

    axum::serve(listener, router).await.unwrap();
}
