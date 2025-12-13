use config::Config;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(get_config);

pub fn get_config() -> AppConfig {
    let app_config = AppConfig::try_load().expect("Failed to load config");
    app_config
}

#[derive(Debug)]
pub struct AppConfig {
    pub inner: Arc<AppConfigInner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigInner {
    pub server: Server,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,
}

impl AppConfig {
    #[cfg(not(feature = "test-util"))]
    pub fn try_load() -> Result<AppConfig, config::ConfigError> {
        Self::try_load0("config/app.yaml")
    }

    #[cfg(feature = "test-util")]
    pub fn try_load() -> Result<AppConfig, config::ConfigError> {
        Self::try_load0("config/app-test.yaml")
    }

    fn try_load0(file: &str) -> Result<AppConfig, config::ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(file))
            .build()?
            .try_deserialize::<AppConfigInner>()?;
        Ok(AppConfig {
            inner: Arc::new(config),
        })
    }
}

impl Deref for AppConfig {
    type Target = AppConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "test-util")]
    #[test]
    fn test_try_load_with_test_util_feature() {
        // 测试当启用test-util特性时，是否能正确加载test配置文件
        let config = AppConfig::try_load().unwrap();
        // 验证配置中的端口是否与app-test.yaml中的配置一致
        assert_eq!(config.server.port, 8081);
    }

    #[cfg(not(feature = "test-util"))]
    #[test]
    fn test_try_load_without_test_util_feature() {
        // 测试当不启用test-util特性时，是否能正确加载默认配置文件
        let config = AppConfig::try_load().unwrap();
        // 验证配置中的端口是否与app.yaml中的配置一致
        assert_eq!(config.server.port, 8080);
    }
}