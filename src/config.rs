use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replace {
    #[serde(default = "get_default_file")]
    pub file: String,
    #[serde(default = "get_default_pattern")]
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "get_default_current_version")]
    pub current_version: String,
    #[serde(default = "get_default_replaces")]
    pub replaces: Vec<Replace>,
}

fn get_default_current_version() -> String {
    "0.1.0".to_string()
}
fn get_default_file() -> String {
    "Cargo.toml".to_string()
}
fn get_default_pattern() -> String {
    "^version\\s*=\\s*\"{{current_version}}\"$".to_string()
}
fn get_default_replaces() -> Vec<Replace> {
    vec![Replace {
        file: get_default_file(),
        pattern: get_default_pattern(),
    }]
}

impl Config {
    fn default() -> Self {
        Self {
            current_version: get_default_current_version(),
            replaces: get_default_replaces(),
        }
    }

    pub async fn write_default(file: &PathBuf) -> io::Result<()> {
        let default = Self::default();
        let yaml = serde_yaml::to_string(&default).expect("Config is always serializable");
        tokio::fs::write(file, yaml.as_bytes()).await
    }

    pub async fn read(file_path: &PathBuf) -> Option<Self> {
        let content = match tokio::fs::read_to_string(file_path).await {
            Ok(c) => c,
            Err(e) => {
                error!(
                    "Failed to read config file '{}': {}",
                    file_path.display(),
                    e
                );
                return None;
            }
        };
        match serde_yaml::from_str::<Self>(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                error!(
                    "Failed to deserialize config file '{}': {}",
                    file_path.display(),
                    e
                );
                None
            }
        }
    }

    pub async fn write(&self, file: &PathBuf) -> io::Result<()> {
        let yaml = serde_yaml::to_string(self).expect("Config is always serializable");
        tokio::fs::write(file, yaml.as_bytes()).await
    }
}
