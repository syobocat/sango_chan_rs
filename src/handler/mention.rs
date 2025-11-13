// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::{sync::Arc, time::Duration};

use chrono::{Local, Timelike};
use rand::seq::IndexedRandom;
use regex::Regex;

use crate::{
    Sango,
    misskey::{
        following::{CreateFollowing, DeleteFollowing},
        notes::{CreateNote, Note},
        users::ShowUser,
    },
};

const MAX_NICKNAME_LENGTH: usize = 15;

pub async fn on_mention(note: Note, sango: Arc<Sango>) -> anyhow::Result<()> {
    let text = &note.text;

    // 特殊処理パターン (時間がかかるので別タスクを建てる)
    if text.contains("フォロー解除して") {
        let user = sango
            .client
            .users_show(ShowUser::by_user_id(&note.user_id))
            .await?;

        let mention = note.user.mention();
        if user.is_following {
            let response = format!("{mention} さよなら、になっちゃうのかな……");
            sango.client.notes_create(note.reply(&response)).await?;
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(10)).await;
                if let Err(e) = sango
                    .client
                    .following_delete(DeleteFollowing::new(&note.user_id))
                    .await
                {
                    log::error!("{e}");
                }
            });
        } else {
            let response = format!("{mention} もともとフォローしてないよー");
            sango.client.notes_create(note.reply(&response)).await?;
        }
        return Ok(());
    } else if text.contains("さんごちゃーん") || text.contains("さんごちゃ〜ん") {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if let Err(e) = sango.client.notes_create(note.reply("は〜い")).await {
                log::error!("{e}");
            }
        });
        return Ok(());
    } else if text.contains("何が好き？") {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if let Err(e) = sango
                .client
                .notes_create(note.reply("チョココーヒー よりもあ・な・た♪"))
                .await
            {
                log::error!("{e}");
            }
            tokio::time::sleep(Duration::from_secs(10)).await;
            if let Err(e) = sango
                .client
                .notes_create(CreateNote::new("さっきのなに……？"))
                .await
            {
                log::error!("{e}");
            }
        });
        return Ok(());
    } else if text.contains("回線速度計測") {
        if note.user_id != sango.admin_id {
            sango
                .client
                .notes_create(note.reply("この機能は使える人が限られてるんだ。ゴメンね"))
                .await?;
            return Ok(());
        }
        sango
            .client
            .notes_create(note.reply("了解。じゃあ計測してくるね"))
            .await?;
        tokio::spawn(async move {
            log::info!("Starting speedtest...");
            let (ping, down, up) = match tokio::task::spawn_blocking(speedtest).await {
                Ok(values) => values,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

            let response = format!(
                "計測かんりょー。下り{down:.2}Mbps、上り{up:.2}Mbps、ping値{ping:.2}msだったよ。……これは速いって言えるのかな？"
            );
            if let Err(e) = sango.client.notes_create(note.reply(&response)).await {
                log::error!("{e}");
            }
        });
        return Ok(());
    } else if text.contains("todo") {
        log::info!("Todo created.");
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(60 * 60 * 3)).await;
            if let Err(e) = sango.client.notes_create(note.reply("これやった？")).await {
                log::error!("{e}");
            }
        });
        return Ok(());
    }

    // 通常パターン
    let response = if text.contains("フォローして") {
        let user = sango
            .client
            .users_show(ShowUser::by_user_id(&note.user_id))
            .await?;

        let mention = note.user.mention();
        if user.is_following {
            let name = sango.savedata.read().await.get_displayname(&note.user);
            &format!("{mention} {name}さん、もうフォローしてるよー")
        } else if user.is_followed {
            let name = note.user.name.as_ref().unwrap_or(&note.user.username);
            sango
                .client
                .following_create(CreateFollowing::new(&note.user_id))
                .await?;
            log::info!("Followed {}.", note.user_id);
            &format!("{mention} フォローバックしたよ、{name}さん。これからよろしくね",)
        } else {
            "……だれ？"
        }
    } else if text.contains("はじめまして") {
        "はじめまして、わたしを見つけてくれてありがとう。これからよろしくね"
    } else if text.contains("こんにちは") {
        "こんにちは、どうしたの？"
    } else if text.contains("自己紹介") || text.contains("あなたは？") {
        // Modified
        "わたしは「3.5Mbps.net」の看板娘、さんご……のクローンです。……めんどうだから、わたしのことも「さんご」でいいよ。\nあなたのことも、教えて欲しいな"
    } else if text.contains("よしよし") || text.contains("なでなで") {
        "わたしの頭なんか撫でて、楽しい？ えっと、あなたが喜んでくれるなら、いいんだけど……"
    } else if text.contains("にゃーん") {
        "にゃ〜ん"
    } else if text.contains("今何時") || text.contains("いまなんじ") {
        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        &format!(
            "いまは {hour}:{minute}:{second} だよ。どうしたの……？ 時計を見る元気もない感じかな？"
        )
    } else if text.contains("罵って") {
        insult()
    } else if text.contains("ちくわ大明神") {
        "…なに？"
    } else if text.contains("ping") {
        "pong？"
    } else if text.contains("って呼んで") || text.contains("と呼んで") {
        match extract_nickname(&text) {
            NicknameResult::NotFound => {
                // 正しくなければ無視
                return Ok(());
            }
            NicknameResult::TooLong => &format!(
                "えぇっと、その名前はちょっと長いかも……\n{MAX_NICKNAME_LENGTH}文字以内にしてほしいな"
            ),
            NicknameResult::Invalid => "えぇっと、その名前はちょっと……だめかも……",
            NicknameResult::Ok(name) => {
                sango
                    .savedata
                    .write()
                    .await
                    .store_nickname(&note.user_id, &name)?;
                &format!(
                    "わかった。これからは{name}さんって呼ぶね\nこれからもよろしくね、{name}さん"
                )
            }
        }
    } else if text.contains("呼び名を忘れて") || text.contains("あだ名を消して") {
        let removed = sango
            .savedata
            .write()
            .await
            .forget_nickname(&note.user_id)?;
        if removed {
            let name = note.user.name.as_ref().unwrap_or(&note.user.username);
            &format!("わかった。これからは{name}さんって呼ぶね\nこれからもよろしくね、{name}さん")
        } else {
            "もともと特別な呼び名は登録されていないみたいだよ"
        }
    } else if text.contains("さんごちゃん？") {
        let name = sango.savedata.read().await.get_displayname(&note.user);
        &format!("どうしたの？ {name}さん")
    } else {
        return Ok(());
    };

    sango.client.notes_create(note.reply(response)).await?;

    Ok(())
}

fn speedtest() -> (f64, f64, f64) {
    let client = reqwest::blocking::Client::new();
    log::info!("Measuring latency...");
    let ping = cfspeedtest::speedtest::test_latency(&client);
    log::info!("Measuring download...");
    let down =
        cfspeedtest::speedtest::test_download(&client, 25_000_000, cfspeedtest::OutputFormat::None);
    log::info!("Measuring upload...");
    let up =
        cfspeedtest::speedtest::test_upload(&client, 25_000_000, cfspeedtest::OutputFormat::None);
    log::info!("Speedtest done.");

    (ping, down, up)
}

enum NicknameResult {
    NotFound,
    TooLong,
    Invalid,
    Ok(String),
}

fn extract_nickname(text: &str) -> NicknameResult {
    let re = Regex::new(r"@\S+\s*(.+)\s*(と呼んで|って呼んで)").unwrap();
    let Some(cap) = re.captures(text) else {
        return NicknameResult::NotFound;
    };
    let Some(extracted) = cap.get(1) else {
        return NicknameResult::NotFound;
    };
    let extracted = extracted.as_str();
    if extracted.chars().count() > MAX_NICKNAME_LENGTH {
        return NicknameResult::TooLong;
    }
    let Some(name) = sanitize_nickname(extracted) else {
        return NicknameResult::Invalid;
    };
    NicknameResult::Ok(name)
}

fn sanitize_nickname(name: &str) -> Option<String> {
    let sanitized = name
        .replace('<', "<\u{200b}") // <center>、<plain>など
        .replace('$', "$\u{200b}") // $[で始まるMFM全般
        .replace("://", ":\u{200b}//") // リンク
        .replace("](", "]\u{200b}(") // リンク
        .replace('#', "#\u{200b}") // ハッシュタグ
        .replace('@', "@\u{200b}") // メンション
        .replace('*', "*\u{200b}") // 太字、イタリック
        .replace("\u{061c}", "") // Arabic letter mark
        .replace("\u{200e}", "") // Left-to-right mark
        .replace("\u{200f}", "") // Right-to-left mark
        .replace("\u{202a}", "") // Left-to-right embedding
        .replace("\u{202b}", "") // Right-to-left embedding
        .replace("\u{202c}", "") // Pop directional formatting
        .replace("\u{202d}", "") // Left-to-right override
        .replace("\u{202e}", "") // Right-to-left override
        .replace("\u{2066}", "") // Left-to-right isolate
        .replace("\u{2067}", "") // Right-to-left isolate
        .replace("\u{2068}", "") // First strong isolate
        .replace("\u{2069}", ""); // Pop directional isolate
    if sanitized.is_empty() || sanitized.chars().all(char::is_whitespace) {
        None
    } else {
        Some(sanitized)
    }
}

fn insult() -> &'static str {
    [
        "変なお願いをするもんだね……",
        "えっと……、ど、どんな風に罵ってほしいとか、ある？",
    ]
    .choose(&mut rand::rng())
    .unwrap()
}
