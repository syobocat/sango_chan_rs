// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::{sync::Arc, time::Duration};

use env_logger::Env;
use rustls::crypto::ring::default_provider;
use tokio::sync::RwLock;

use crate::{
    config::Config,
    misskey::{MisskeyClient, notes::CreateNote},
    savedata::SaveData,
    websocket::MisskeyWebsocket,
};

mod config;
mod handler;
mod misskey;
mod savedata;
mod websocket;

struct Sango {
    client: MisskeyClient,
    self_id: String,
    admin_id: String,
    savedata: RwLock<SaveData>,
}

impl Sango {
    async fn new(config: &Config) -> anyhow::Result<Self> {
        let client = MisskeyClient::new(&config.host, &config.token);
        let self_id = client.get_id_self().await?;
        let savedata = SaveData::load().unwrap_or_else(|_| {
            log::warn!("savedata.json is not found or cannot be read; Creating new one...");
            SaveData::default()
        });
        let savedata = RwLock::new(savedata);
        Ok(Self {
            client,
            self_id,
            savedata,
            admin_id: config.admin.clone(),
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::new().default_filter_or("info");
    env_logger::init_from_env(env);

    log::info!("Booting up...");
    default_provider().install_default().unwrap();

    let conf = Config::load()?;
    let sango = Sango::new(&conf).await?;
    let sango = Arc::new(sango);

    log::info!("Authorized as {}.", sango.self_id);

    loop {
        let sango = Arc::clone(&sango);
        if let Err(e) = main_loop(sango, &conf).await {
            log::error!("{e}");
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }
}

async fn main_loop(sango: Arc<Sango>, conf: &Config) -> anyhow::Result<()> {
    let sango = Arc::clone(&sango);
    let mut ws = MisskeyWebsocket::new(&conf.host, &conf.token).await?;

    sango
        .client
        .notes_create(CreateNote::new("うーん、うとうとしちゃってたみたい……？"))
        .await?;

    loop {
        let next = ws.next().await?; // 接続が切れたらloopを抜ける

        // Fire and forget
        tokio::spawn(handler::handle(next, Arc::clone(&sango)));
    }
}
