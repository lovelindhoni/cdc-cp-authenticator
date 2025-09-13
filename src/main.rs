mod authenticator;
mod handlers;
mod utils;

use crate::authenticator::Authenticator;
use crate::handlers::{auth_codechef, auth_codeforces, auth_leetcode, root};
use axum::{
    Router,
    routing::{get, post},
};

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/auth/leetcode", post(auth_leetcode))
        .route("/auth/codechef", post(auth_codechef))
        .route("/auth/codeforces", post(auth_codeforces))
        .with_state(Authenticator::new())
}

#[cfg(feature = "local")]
use tracing::info;
#[cfg(feature = "local")]
const PORT: u32 = 3000;

#[cfg(feature = "local")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = create_app();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .unwrap();

    info!("CP-authenticator running on port {}", PORT);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "local"))]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    tracing_subscriber::fmt::init();
    let app = create_app();
    Ok(app.into())
}
