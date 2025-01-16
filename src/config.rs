//! Tanic Config module

use directories::ProjectDirs;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use http::Uri;
use serde::{Deserialize, Serialize};

use crate::Result;

/// Represents a named set of connection details for an Iceberg catalog
#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogConnectionDetails {
    /// The name of this connection
    pub name: String,

    /// The URI of this connection
    #[serde(with = "http_serde::uri")]
    pub uri: Uri,
    // Type?
}

/// persistable user config.
///
/// Loaded in at application startup from $CONFIG/tanic/tanic.toml
#[derive(Debug, Serialize, Deserialize)]
pub struct TanicConfig {
    /// list of known connections
    pub library: Vec<CatalogConnectionDetails>,
}

impl Default for TanicConfig {
    fn default() -> TanicConfig {
        TanicConfig {
            library: Vec::new(),
        }
    }
}

impl TanicConfig {
    /// Load config by merging standard sources of config
    ///
    /// Priority: defaults < config file < env < args
    pub fn load() -> Result<TanicConfig> {
        let mut figment = Figment::from(Serialized::defaults(TanicConfig::default()));

        if let Some(proj_dirs) = ProjectDirs::from("com", "Tanic", "Tanic") {
            let config_dir = proj_dirs.config_dir();

            figment = figment.merge(Toml::file(config_dir.join("tanic.toml")))
        }

        Ok(figment.merge(Env::prefixed("TANIC_")).extract()?)
    }
}
