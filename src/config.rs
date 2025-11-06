// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub host: String,
    pub admin: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let file = std::fs::read_to_string("config.toml").context("Failed to load config")?;
        let config = toml::from_str(&file).context("Failed to parse config")?;
        Ok(config)
    }
}
