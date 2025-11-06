// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::sync::Arc;

use crate::{
    Sango,
    misskey::{notes::Note, users::User},
    websocket::{EventBodyType, MisskeyWebsocket},
};

mod followed;
mod mention;
mod note;

pub async fn handle(mut ws: MisskeyWebsocket, sango: Arc<Sango>) -> anyhow::Result<()> {
    loop {
        let next = ws.next().await?;
        match next.event_type {
            EventBodyType::Followed => {
                let user: User = match serde_json::from_value(next.body) {
                    Ok(user) => user,
                    Err(e) => {
                        log::error!("{e}");
                        continue;
                    }
                };
                if user.is_bot || user.id == sango.self_id {
                    // BOTを無視
                    continue;
                }
                log::debug!("Received a follow.");
                followed::on_follow(user, &sango).await?;
            }
            EventBodyType::Mention => {
                let note: Note = match serde_json::from_value(next.body) {
                    Ok(note) => note,
                    Err(e) => {
                        log::error!("{e}");
                        continue;
                    }
                };
                if note.user.is_bot || note.user.id == sango.self_id {
                    // BOTを無視
                    continue;
                }
                log::debug!("Received a mention.");
                let sango = Arc::clone(&sango);
                mention::on_mention(note, sango).await?;
            }
            EventBodyType::Note => {
                let note: Note = match serde_json::from_value(next.body) {
                    Ok(note) => note,
                    Err(e) => {
                        log::error!("{e}");
                        continue;
                    }
                };
                if note.user.is_bot || note.user.id == sango.self_id {
                    // BOTを無視
                    continue;
                }
                if note.mentions.contains(&sango.self_id) {
                    // メンションはEventBodyType::Mentionで処理するので無視
                    continue;
                }
                log::debug!("Received a note.");
                note::on_note(note, &sango).await?;
            }
            _ => {}
        }
    }
}
