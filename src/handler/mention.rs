// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::time::Duration;

use chrono::{Local, Timelike};
use rand::seq::IndexedRandom;
use regex::Regex;

use crate::{
    Sango,
    handler::Handler,
    misskey::{
        following::{CreateFollowing, DeleteFollowing},
        notes::{CreateNote, Note},
        users::ShowUser,
    },
};

const MAX_NICKNAME_LENGTH: usize = 15;

pub struct HandleMention;
impl Handler for HandleMention {
    fn gate(&self, note: &Note, sango: &Sango) -> bool {
        !note.user.is_bot// BOTを無視
        && note.user.id != sango.self_id // 自身を無視
    }

    async fn action(&self, note: &Note, sango: &Sango) -> anyhow::Result<()> {
        let _ = HandleFollow.handle(note, sango).await
            || HandleUnFollow.handle(note, sango).await
            || HandleAiScream1.handle(note, sango).await
            || HandleAiScream2.handle(note, sango).await
            || HandleSpeedtest.handle(note, sango).await
            || HandleTodo.handle(note, sango).await
            || HandleMeet.handle(note, sango).await
            || HandleHello.handle(note, sango).await
            || HandleIntro.handle(note, sango).await
            || HandlePat.handle(note, sango).await
            || HandleMeow.handle(note, sango).await
            || HandleTime.handle(note, sango).await
            || HandleInsult.handle(note, sango).await
            || HandleChikuwa.handle(note, sango).await
            || HandlePing.handle(note, sango).await
            || HandleSetNickname.handle(note, sango).await
            || HandleForgetNickname.handle(note, sango).await;
        Ok(())
    }
}

struct HandleFollow;
impl Handler for HandleFollow {
    const KEYWORDS: &[&str] = &["フォローして"];
    async fn respond(&self, note: &Note, sango: &Sango) -> anyhow::Result<String> {
        let user = sango
            .client
            .users_show(ShowUser::by_user_id(&note.user_id))
            .await?;
        let mention = note.user.mention();

        if user.is_following {
            let name = sango.savedata.read().await.get_displayname(&note.user);
            Ok(format!("{mention} {name}さん、もうフォローしてるよー"))
        } else if user.is_followed {
            let name = note.user.name.as_ref().unwrap_or(&note.user.username);
            sango
                .client
                .following_create(CreateFollowing::new(&note.user_id))
                .await?;
            log::info!("Followed {}.", note.user_id);
            Ok(format!(
                "{mention} フォローバックしたよ、{name}さん。これからよろしくね",
            ))
        } else {
            Ok("……だれ？".to_owned())
        }
    }
}

struct HandleUnFollow;
impl Handler for HandleUnFollow {
    const KEYWORDS: &[&str] = &["フォロー解除して"];
    async fn action(&self, note: &Note, sango: &Sango) -> anyhow::Result<()> {
        let user = sango
            .client
            .users_show(ShowUser::by_user_id(&note.user_id))
            .await?;

        let mention = note.user.mention();
        if user.is_following {
            let response = format!("{mention} さよなら、になっちゃうのかな……");
            sango.client.notes_create(note.reply(&response)).await?;
            tokio::time::sleep(Duration::from_secs(10)).await;
            sango
                .client
                .following_delete(DeleteFollowing::new(&note.user_id))
                .await?;
        } else {
            let response = format!("{mention} もともとフォローしてないよー");
            sango.client.notes_create(note.reply(&response)).await?;
        }
        Ok(())
    }
}

struct HandleAiScream1;
impl Handler for HandleAiScream1 {
    const KEYWORDS: &[&str] = &["さんごちゃーん", "さんごちゃ〜ん"];
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok("は〜い".to_owned())
    }
}

struct HandleAiScream2;
impl Handler for HandleAiScream2 {
    const KEYWORDS: &[&str] = &["何が好き？"];
    async fn action(&self, note: &Note, sango: &Sango) -> anyhow::Result<()> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        sango
            .client
            .notes_create(note.reply("チョココーヒー よりもあ・な・た♪"))
            .await?;
        tokio::time::sleep(Duration::from_secs(10)).await;
        sango
            .client
            .notes_create(CreateNote::new("さっきのなに……？"))
            .await?;
        Ok(())
    }
}

struct HandleSpeedtest;
impl Handler for HandleSpeedtest {
    const KEYWORDS: &[&str] = &["回線速度計測"];
    async fn respond(&self, note: &Note, sango: &Sango) -> anyhow::Result<String> {
        if note.user_id != sango.admin_id {
            return Ok("この機能は使える人が限られてるんだ。ゴメンね".to_owned());
        }

        sango
            .client
            .notes_create(note.reply("了解。じゃあ計測してくるね"))
            .await?;

        log::info!("Starting speedtest...");
        let (ping, down, up) = tokio::task::spawn_blocking(speedtest).await?;

        Ok(format!(
            "計測かんりょー。下り{down:.2}Mbps、上り{up:.2}Mbps、ping値{ping:.2}msだったよ。……これは速いって言えるのかな？"
        ))
    }
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

struct HandleTodo;
impl Handler for HandleTodo {
    const KEYWORDS: &[&str] = &["todo"];
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        log::info!("Todo created.");
        tokio::time::sleep(Duration::from_secs(60 * 60 * 3)).await;
        Ok("これやった？".to_owned())
    }
}

struct HandleMeet;
impl Handler for HandleMeet {
    const KEYWORDS: &[&str] = &["はじめまして"];
    const RESPONSE: &str = "はじめまして、わたしを見つけてくれてありがとう。これからよろしくね";
}

struct HandleHello;
impl Handler for HandleHello {
    const KEYWORDS: &[&str] = &["こんにちは"];
    const RESPONSE: &str = "こんにちは、どうしたの？";
}

struct HandleIntro;
impl Handler for HandleIntro {
    const KEYWORDS: &[&str] = &["自己紹介", "あなたは？"];
    const RESPONSE: &str = "わたしは「3.5Mbps.net」の看板娘、さんご……のクローンです。……めんどうだから、わたしのことも「さんご」でいいよ。\nあなたのことも、教えて欲しいな";
}

struct HandlePat;
impl Handler for HandlePat {
    const KEYWORDS: &[&str] = &["よしよし", "なでなで"];
    const RESPONSE: &str =
        "わたしの頭なんか撫でて、楽しい？ えっと、あなたが喜んでくれるなら、いいんだけど……";
}

struct HandleMeow;
impl Handler for HandleMeow {
    const KEYWORDS: &[&str] = &["にゃーん"];
    const RESPONSE: &str = "にゃ〜ん";
}

struct HandleTime;
impl Handler for HandleTime {
    const KEYWORDS: &[&str] = &["今何時", "いまなんじ"];
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        Ok(format!(
            "いまは {hour}:{minute}:{second} だよ。どうしたの……？ 時計を見る元気もない感じかな？"
        ))
    }
}

struct HandleInsult;
impl Handler for HandleInsult {
    const KEYWORDS: &[&str] = &["罵って"];
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        let chosen = [
            "変なお願いをするもんだね……",
            "えっと……、ど、どんな風に罵ってほしいとか、ある？",
        ]
        .choose(&mut rand::rng())
        .unwrap();
        Ok(chosen.to_owned().to_owned())
    }
}

struct HandleChikuwa;
impl Handler for HandleChikuwa {
    const KEYWORDS: &[&str] = &["ちくわ大明神"];
    const RESPONSE: &str = "…なに？";
}

struct HandlePing;
impl Handler for HandlePing {
    const KEYWORDS: &[&str] = &["ping"];
    const RESPONSE: &str = "pong？";
}

struct HandleSetNickname;
impl Handler for HandleSetNickname {
    const KEYWORDS: &[&str] = &["って呼んで", "と呼んで"];
    async fn respond(&self, note: &Note, sango: &Sango) -> anyhow::Result<String> {
        match extract_nickname(&note.text) {
            NicknameResult::NotFound => {
                // 正しくなければ無視
                Ok(String::new())
            }
            NicknameResult::TooLong => Ok(format!(
                "えぇっと、その名前はちょっと長いかも……\n{MAX_NICKNAME_LENGTH}文字以内にしてほしいな"
            )),
            NicknameResult::Invalid => Ok("えぇっと、その名前はちょっと……だめかも……".to_owned()),
            NicknameResult::Ok(name) => {
                sango
                    .savedata
                    .write()
                    .await
                    .store_nickname(&note.user_id, &name)?;
                Ok(format!(
                    "わかった。これからは{name}さんって呼ぶね\nこれからもよろしくね、{name}さん"
                ))
            }
        }
    }
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

enum NicknameResult {
    NotFound,
    TooLong,
    Invalid,
    Ok(String),
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

struct HandleForgetNickname;
impl Handler for HandleForgetNickname {
    const KEYWORDS: &[&str] = &["呼び名を忘れて", "あだ名を消して"];
    async fn respond(&self, note: &Note, sango: &Sango) -> anyhow::Result<String> {
        let removed = sango
            .savedata
            .write()
            .await
            .forget_nickname(&note.user_id)?;
        if removed {
            let name = note.user.name.as_ref().unwrap_or(&note.user.username);
            Ok(format!(
                "わかった。これからは{name}さんって呼ぶね\nこれからもよろしくね、{name}さん"
            ))
        } else {
            Ok("もともと特別な呼び名は登録されていないみたいだよ".to_owned())
        }
    }
}
