// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::sync::Arc;

use crate::{
    Sango,
    handler::{mention::HandleMention, note::HandleNote},
    misskey::{notes::Note, users::User},
    websocket::{EventBody, EventBodyType},
};

mod followed;
mod mention;
mod note;

pub trait Handler {
    // 反応する単語
    const KEYWORDS: &[&str] = &[];

    // 追加条件
    fn cond(&self, _note: &Note) -> bool {
        true
    }

    fn gate(&self, note: &Note, _sango: &Sango) -> bool {
        let keyword_check = Self::KEYWORDS
            .iter()
            .any(|keyword| note.text.contains(keyword));
        keyword_check && self.cond(note)
    }

    // `action`、`respond`、`RESPONSE`のいずれかを実装する
    const RESPONSE: &str = "";
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        Ok(Self::RESPONSE.to_owned())
    }
    async fn action(&self, note: &Note, sango: &Sango) -> anyhow::Result<()> {
        let response = self.respond(note, sango).await?;
        if !response.is_empty() {
            sango.client.request(note.reply(&response)).await?;
        }
        Ok(())
    }

    // 上書きしない
    async fn handle(&self, note: &Note, sango: &Sango) -> bool {
        if self.gate(note, sango) {
            if let Err(e) = self.action(note, sango).await {
                log::error!("{e}");
            }
            true
        } else {
            false
        }
    }
}

pub async fn handle(event: EventBody, sango: Arc<Sango>) {
    match event.event_type {
        EventBodyType::Followed => {
            let user: User = match serde_json::from_value(event.body) {
                Ok(user) => user,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };
            log::debug!("Received a follow.");
            followed::on_follow(user, &sango).await;
        }
        EventBodyType::Mention => {
            let note: Note = match serde_json::from_value(event.body) {
                Ok(note) => note,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };
            log::debug!("Received a mention.");
            HandleMention.handle(&note, &sango).await;
        }
        EventBodyType::Note => {
            let note: Note = match serde_json::from_value(event.body) {
                Ok(note) => note,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };
            log::debug!("Received a note.");
            HandleNote.handle(&note, &sango).await;
        }
        _ => {}
    }
}
