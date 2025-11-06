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

    loop {
        let sango = Arc::clone(&sango);
        log::info!("Authorized as {}.", sango.self_id);
        let ws = match MisskeyWebsocket::new(&conf.host, &conf.token).await {
            Ok(ws) => ws,
            Err(e) => {
                log::warn!("{e}");
                log::warn!("Failed to connect to WebSocket. Retrying in 30 secs...");
                // 接続できなかったら30秒待って再接続を試みる
                tokio::time::sleep(Duration::from_secs(30)).await;
                continue;
            }
        };

        if let Err(e) = sango
            .client
            .notes_create(CreateNote::new("うーん、うとうとしちゃってたみたい……？"))
            .await
        {
            log::error!("{e}");
            // 接続できなかったら30秒待って再接続を試みる
            tokio::time::sleep(Duration::from_secs(30)).await;
            continue;
        }

        // 値が返ってきたら接続が死んでいるので、loopにより再接続を試みる
        if let Err(e) = handler::handle(ws, sango).await {
            eprintln!("ERR: {e}");
        }
    }
}
