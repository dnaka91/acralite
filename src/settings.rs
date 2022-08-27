use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::dirs::DIRS;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub auth: Auth,
    #[serde(default)]
    pub tracing: Option<Tracing>,
}

#[derive(Clone, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct Tracing {
    pub otlp: Otlp,
}

#[derive(Clone, Deserialize)]
pub struct Otlp {
    pub endpoint: String,
}

pub fn load() -> Result<Settings> {
    let buf = fs::read(DIRS.settings_file()).context("failed reading settings file")?;
    toml::from_slice(&buf).context("failed parsing settings")
}
