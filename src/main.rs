//! altare-api：跑在 Altare 上的小 API 后端，只服务自己。
//!
//! 约定：
//! - 监听 0.0.0.0:$PORT（Altare 给 PORT，本地默认 3000）。
//! - GET /health 是健康检查，部署设置里配这个路径。
//! - 配置全走环境变量，不要把密钥写进仓库。

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use std::net::SocketAddr;
use tracing::info;

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn index() -> Json<Value> {
    Json(json!({
        "service": "altare-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "altare_api=info,tower_http=info".into()),
        )
        .init();

    // 在这里加你的接口：.route("/xxx", get(handler)) / post(handler)
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
