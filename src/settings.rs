use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::dirs::DIRS;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub auth: Auth,
}

#[derive(Clone, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

pub fn load() -> Result<Settings> {
    let buf = fs::read(DIRS.settings_file()).context("failed reading settings file")?;
    toml::from_slice(&buf).context("failed parsing settings")
}
