use anyhow::{bail, Result};
use serde::Deserialize;

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
    let locations = &[
        concat!("/etc/", env!("CARGO_PKG_NAME"), "/config.toml"),
        concat!("/app/", env!("CARGO_PKG_NAME"), ".toml"),
        concat!(env!("CARGO_PKG_NAME"), ".toml"),
    ];
    let buf = locations.iter().find_map(|loc| std::fs::read(loc).ok());

    match buf {
        Some(buf) => Ok(toml::from_slice(&buf)?),
        None => bail!("failed finding settings"),
    }
}
