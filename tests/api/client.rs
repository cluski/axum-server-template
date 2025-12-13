use rootcause::Report;

use axum_server_template::AppConfig;

pub struct TestClient {
    client: reqwest::Client,
    config: AppConfig,
}

impl TestClient {
    pub fn new() -> Result<Self, Report> {
        let config = AppConfig::try_load()?;
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }

    #[allow(unused)]
    pub async fn get<T>(&self, path: &str) -> Result<T, Report>
    where
        T: serde::de::DeserializeOwned,
    {
        let base_url = format!("http://localhost:{}", self.config.inner.server.port);
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
        let base_url = format!("http://localhost:{}", self.config.inner.server.port);
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
