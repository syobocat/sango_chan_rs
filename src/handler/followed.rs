// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use crate::{
    Sango,
    misskey::{notes::CreateNote, users::User},
};

pub async fn on_follow(user: User, sango: &Sango) -> anyhow::Result<()> {
    let mention = user.mention();
    let text = format!(
        "フォローありがとうございます、{mention}さん\n「フォローして」とメンションしながら投稿すると、フォローバックするよ"
    );
    sango.client.notes_create(CreateNote::new(&text)).await?;
    Ok(())
}
