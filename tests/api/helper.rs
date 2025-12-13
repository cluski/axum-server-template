use axum_server_template::{AppConfig, start_server_with_listener};
use rootcause::prelude::*;
use tokio::net::TcpListener;

pub struct TestHelper {
    pub client: TestClient,
    #[allow(dead_code)]
    pub app_config: AppConfig,
    #[allow(dead_code)]
    port: u16,
}

impl TestHelper {
    pub async fn new_and_spawn_server() -> Result<Self, Report> {
        let app_config = AppConfig::try_load()?;
        let listener = TcpListener::bind("127.0.0.1:0").await?;

        let port = listener.local_addr()?.port();

        tokio::spawn(async move {
            let _ = start_server_with_listener(listener).await;
        });

        Ok(Self {
            client: TestClient::new(port as usize)?,
            app_config,
            port,
        })
    }
}

pub struct TestClient {
    client: reqwest::Client,
    port: usize,
}

impl TestClient {
    pub fn new(port: usize) -> Result<Self, Report> {
        Ok(Self {
            client: reqwest::Client::new(),
            port,
        })
    }

    #[allow(unused)]
    pub async fn get<T>(&self, path: &str) -> Result<T, Report>
    where
        T: serde::de::DeserializeOwned,
    {
        let base_url = format!("http://localhost:{}", self.port);
        let url = format!("{}{}", base_url, path);
        self.client
            .get(url)
            .send()
            .await?
            .json::<T>()
            .await
            .map_err(Into::into)
    }

    #[allow(unused)]
    pub async fn post<T>(&self, path: &str, body: &[u8]) -> Result<T, Report>
    where
        T: serde::de::DeserializeOwned,
    {
        let base_url = format!("http://localhost:{}", self.port);
        let url = format!("{}{}", base_url, path);
        self.client
            .post(url)
            .body(body.to_vec())
            .send()
            .await?
            .json::<T>()
            .await
            .map_err(Into::into)
    }
}
