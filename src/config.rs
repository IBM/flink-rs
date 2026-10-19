// Copyright IBM Corp. 2025
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub timeout_secs: u64,
    pub output: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "http://localhost:8081".into(),
            timeout_secs: 30,
            output: OutputFormat::default(),
        }
    }
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("flink-cli")
            .join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::path();
        if !path.exists() {
            return Self::default();
        }
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating config dir {}", parent.display()))?;
        }
        let content = toml::to_string_pretty(self).context("serialising config")?;
        std::fs::write(&path, content)
            .with_context(|| format!("writing config to {}", path.display()))?;
        Ok(())
    }

    /// Merge CLI overrides: values from flags win over stored config.
    pub fn apply_overrides(&mut self, host: Option<String>, output: Option<OutputFormat>) {
        if let Some(h) = host {
            self.host = h;
        }
        if let Some(o) = output {
            self.output = o;
        }
    }
}

/// Resolve the effective config given global CLI flags.
/// Loads from disk, then applies any flag overrides.
pub fn resolve(host: Option<String>, output: Option<OutputFormat>) -> Config {
    let mut cfg = Config::load();
    cfg.apply_overrides(host, output);
    cfg
}
