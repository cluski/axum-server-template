mod config;
mod router;
mod tracing;

use std::net::SocketAddr;

use crate::{config::APP_CONFIG, tracing::init_tracing};
use ::tracing::info;
use rootcause::prelude::Report;
use tokio::net::TcpListener;

pub use config::AppConfig;
pub use router::get_router;

#[allow(unused)]
pub async fn start_server() -> Result<(), Report> {
    // 设置监听地址
    let addr = SocketAddr::from(([0, 0, 0, 0], APP_CONFIG.server.port as _));
    // 绑定 TCP 监听器
    let listener = TcpListener::bind(addr).await?;
    // 启动服务器
    start_server_with_listener(listener).await
}

#[allow(unused)]
pub async fn start_server_with_listener(listener: TcpListener) -> Result<(), Report> {
    let _guard = init_tracing();

    info!("Listening on http://{}", listener.local_addr()?);

    // 构建我们的应用路由
    let app = get_router();

    let port = listener.local_addr()?.port();
    // 启动服务器
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
