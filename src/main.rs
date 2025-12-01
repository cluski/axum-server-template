use axum_server_template::start_server;
use rootcause::Report;

#[tokio::main]
async fn main() -> Result<(), Report> {
    start_server().await?;
    Ok(())
}
