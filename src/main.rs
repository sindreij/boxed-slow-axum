use axum::{prelude::*, routing::BoxRoute};

#[tokio::main]
async fn main() {
    start().await;
}

async fn start(){
    let app = route("/a", get(get_a))
        .route("/b", get(get_b))
        .route("/c", get(get_c))
        .route("/d", get(get_d))
        .route("/e", get(get_e))
        .route("/f", get(get_f))
        .route("/g", get(get_g))
        .route("/h", get(get_h))
        // Remove boxed, and the compile-time goes from around a minute to around a second.
        .boxed();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// GET /api/v1/apps
async fn get_a() -> &'static str {
    "ok"
}

// GET /api/v1/apps/{git_owner}/{git_name}
async fn get_b() -> &'static str {
    "ok"
}

// GET /api/v1/deployment/{id}
async fn get_c() -> &'static str {
    "ok"
}
// GET /api/v1/builds
async fn get_aa() -> &'static str {
    "ok"
}

// GET /api/v1/builds/{build_id}
async fn get_e() -> &'static str {
    "ok"
}

// GET /api/v1/secrets/publickey
async fn get_h() -> &'static str {
    "ok"
}

async fn get_f() -> &'static str {
    "ok"
}

async fn get_g() -> &'static str {
    "ok"
}

async fn get_d() -> &'static str {
    "ok"
}
