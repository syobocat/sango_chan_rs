// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use crate::{
    Sango,
    misskey::{notes::CreateNote, users::User},
};

// TODO: なんかうまいことしてここもHandlerにしたい
pub async fn on_follow(user: User, sango: &Sango) {
    if user.is_bot || user.id == sango.self_id {
        // BOTを無視
        return;
    }

    let mention = user.mention();
    let text = format!(
        "フォローありがとうございます、{mention}さん\n「フォローして」とメンションしながら投稿すると、フォローバックするよ"
    );
    if let Err(e) = sango.client.notes_create(CreateNote::new(&text)).await {
        log::error!("{e}");
    }
}
