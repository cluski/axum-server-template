mod config;
mod tracing;

use std::net::SocketAddr;

use crate::{config::APP_CONFIG, tracing::init_tracing};
use ::tracing::info;
use axum::Router;
use rootcause::prelude::Report;
use tokio::net::TcpListener;

#[allow(unused)]
pub async fn start_server() -> Result<(), Report> {
    let _guard = init_tracing();

    // 构建我们的应用路由
    let app = Router::new();

    // 设置监听地址
    let addr = SocketAddr::from(([0, 0, 0, 0], APP_CONFIG.server.port as _));
    info!("Listening on http://{}", addr);

    // 绑定 TCP 监听器
    let listener = TcpListener::bind(addr).await.unwrap();

    // 启动服务器
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
