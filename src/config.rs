//! Load and validate the YAML webhook configuration.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::behaviors;
use crate::color;

/// One webhook: which behavior to run, in which colour, with what tuning.
///
/// `Serialize` so the CLI can POST it to a running server's `/run` endpoint;
/// `Default` so CLI commands can build one with `..Default::default()`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebhookSpec {
    pub behavior: String,
    #[serde(default = "default_color")]
    pub color: String,

    // Optional per-behavior tuning; `None` => the behavior's default is used.
    pub times: Option<u32>,
    pub on: Option<f64>,
    pub off: Option<f64>,
    pub speed: Option<u8>,
    pub repeat: Option<u8>,
    pub duration: Option<u8>,
    pub wave_type: Option<u8>,
}

fn default_color() -> String {
    "red".to_string()
}

/// Top-level config: where to listen and the webhook map.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub webhooks: BTreeMap<String, WebhookSpec>,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8765
}

/// Read and validate a YAML config file.
pub fn load_config(path: &Path) -> Result<ServerConfig, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("could not read config {}: {e}", path.display()))?;
    let config: ServerConfig = serde_yaml::from_str(&text)
        .map_err(|e| format!("invalid config in {}: {e}", path.display()))?;

    // Validate behavior names and colours up front so the server fails fast.
    for (name, spec) in &config.webhooks {
        if !behaviors::is_known(&spec.behavior) {
            let known = behaviors::BEHAVIORS
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!(
                "webhook {name:?}: unknown behavior {:?}; choose one of {known}",
                spec.behavior
            ));
        }
        color::parse_color(&spec.color).map_err(|e| format!("webhook {name:?}: {e}"))?;
    }

    Ok(config)
}
