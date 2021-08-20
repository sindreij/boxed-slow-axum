use axum::handler::get;
use axum::Router;

#[tokio::main]
async fn main() {
    start().await;
}

async fn start() {
    let app = Router::new()
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        .route("/a", get(get_a))
        // Remove boxed, and the `cargo check` goes from multiple minutes to around a second.
        .boxed();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_a() -> &'static str {
    "ok"
}
