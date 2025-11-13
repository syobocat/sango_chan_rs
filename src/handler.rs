// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::sync::Arc;

use crate::{
    Sango,
    misskey::{notes::Note, users::User},
    websocket::{EventBody, EventBodyType},
};

mod followed;
mod mention;
mod note;

pub enum HandleResult {
    Handled,
    Skipped,
    IgnorableError,
}

pub async fn handle(event: EventBody, sango: Arc<Sango>) -> anyhow::Result<HandleResult> {
    match event.event_type {
        EventBodyType::Followed => {
            let user: User = match serde_json::from_value(event.body) {
                Ok(user) => user,
                Err(e) => {
                    log::error!("{e}");
                    return Ok(HandleResult::IgnorableError);
                }
            };
            if user.is_bot || user.id == sango.self_id {
                // BOTを無視
                return Ok(HandleResult::Skipped);
            }
            log::debug!("Received a follow.");
            followed::on_follow(user, &sango).await?;
        }
        EventBodyType::Mention => {
            let note: Note = match serde_json::from_value(event.body) {
                Ok(note) => note,
                Err(e) => {
                    log::error!("{e}");
                    return Ok(HandleResult::IgnorableError);
                }
            };
            if note.user.is_bot || note.user.id == sango.self_id {
                // BOTを無視
                return Ok(HandleResult::Skipped);
            }
            log::debug!("Received a mention.");
            let sango = Arc::clone(&sango);
            mention::on_mention(note, sango).await?;
        }
        EventBodyType::Note => {
            let note: Note = match serde_json::from_value(event.body) {
                Ok(note) => note,
                Err(e) => {
                    log::error!("{e}");
                    return Ok(HandleResult::IgnorableError);
                }
            };
            if note.user.is_bot || note.user.id == sango.self_id {
                // BOTを無視
                return Ok(HandleResult::Skipped);
            }
            if note.mentions.contains(&sango.self_id) {
                // メンションはEventBodyType::Mentionで処理するので無視
                return Ok(HandleResult::Skipped);
            }
            log::debug!("Received a note.");
            note::on_note(note, &sango).await?;
        }
        _ => {}
    }

    Ok(HandleResult::Handled)
}
