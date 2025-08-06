use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application configuration structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub config_version: u32,
    pub settings: Settings,
    pub features: HashMap<String, bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub retries: u32,
    pub timeout_ms: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            config_version: 2,
            settings: Settings {
                retries: 5,
                timeout_ms: 5000,
            },
            features: {
                let mut features = HashMap::new();
                features.insert("beta_feature_x".to_string(), true);
                features
            },
        }
    }
}
