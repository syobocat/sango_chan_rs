// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use rand::seq::IndexedRandom;

use crate::{Sango, handler::Handler, misskey::notes::Note};

pub struct HandleNote;
impl Handler for HandleNote {
    fn gate(&self, note: &Note, sango: &Sango) -> bool {
        !note.user.is_bot // BOTを無視
        && note.user.id != sango.self_id // 自身を無視
        && !note.mentions.contains(&sango.self_id) // メンションはEventBodyType::Mentionで処理するので無視
    }

    async fn action(&self, note: &Note, sango: &Sango) -> anyhow::Result<()> {
        let _ = HandlePain.handle(note, sango).await
            || HandleTired.handle(note, sango).await
            || HandleGoWork.handle(note, sango).await
            || HandleLeaveWork.handle(note, sango).await
            || HandleNullpo.handle(note, sango).await
            || HandleCall.handle(note, sango).await
            || HandleSleepy.handle(note, sango).await
            || HandleGoodMorning.handle(note, sango).await
            || HandleGoodNight.handle(note, sango).await
            || HandleLateMorning.handle(note, sango).await
            || HandleMeow.handle(note, sango).await
            || HandleSleepAgain.handle(note, sango).await;
        Ok(())
    }
}

struct HandlePain;
impl Handler for HandlePain {
    const KEYWORDS: &[&str] = &["つらい", "つらすぎ"];
    const RESPONSE: &str = "つらいときは、甘えてもいいんだよ？";
}

struct HandleTired;
impl Handler for HandleTired {
    const KEYWORDS: &[&str] = &[
        "疲れた",
        "つかれた",
        "疲れてる",
        "つかれてる",
        "疲れている",
        "つかれている",
    ];
    const RESPONSE: &str = "ひとやすみ、する？ それとも、わたしが癒してあげよっか？";
}

struct HandleGoWork;
impl Handler for HandleGoWork {
    const KEYWORDS: &[&str] = &["出勤"];
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        let response = [
            "お仕事、頑張ってきてね。わたし、帰ってくるの、待ってるから……",
            "お仕事は大事だけど、あんまり無理はしないでね？",
            "お仕事とわたし、どっちが大事なんだろう……。まぁ、わたしにはロイちゃんがいるから、いい……のかな？\n……あっ、ち、違う！ これは違くて…！ なんでもないから……！",
        ].choose(&mut rand::rng()).unwrap();
        Ok(response.to_owned().to_owned())
    }
}

struct HandleLeaveWork;
impl Handler for HandleLeaveWork {
    const KEYWORDS: &[&str] = &["退勤"];
    const RESPONSE: &str =
        "お仕事終わったの？ お疲れさま～。 ……わたしの癒し、必要かな？ 必要なら、いつでも言ってね";
}

struct HandleNullpo;
impl Handler for HandleNullpo {
    const KEYWORDS: &[&str] = &["ぬるぽ"];
    fn cond(&self, _note: &Note) -> bool {
        rand::random_bool(1.0 / 3.0)
    }
    const RESPONSE: &str = "ガッ";
}

struct HandleCall;
impl Handler for HandleCall {
    const KEYWORDS: &[&str] = &["さんごちゃん"];
    fn cond(&self, note: &Note) -> bool {
        !note.text.trim_end().ends_with("さんごちゃん") // 「さんごちゃん」以降に文字がある場合のみ
            && rand::random_bool(1.0 / 3.0)
    }
    async fn respond(&self, note: &Note, sango: &Sango) -> anyhow::Result<String> {
        let name = sango.savedata.read().await.get_displayname(&note.user);
        Ok(format!("呼んだ？ {name}さん"))
    }
}

struct HandleSleepy;
impl Handler for HandleSleepy {
    const KEYWORDS: &[&str] = &["眠い", "眠たい", "ねむ"];
    fn cond(&self, note: &Note) -> bool {
        if note.text.contains("ねむ") {
            note.reply_id.is_none() && !note.text.contains("くない")
        } else {
            note.reply_id.is_none()
        }
    }
    const RESPONSE: &str = "なるほど、眠いんだね。……我慢はよくないよ？ 欲には素直にならないと";
}

struct HandleGoodMorning;
impl Handler for HandleGoodMorning {
    const KEYWORDS: &[&str] = &["おはよ"];
    fn cond(&self, note: &Note) -> bool {
        note.reply_id.is_none()
    }
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        let state = [
            "よく眠れたよ～。元気いーっぱい",
            "あんまり寝れなかったかな……。まぁ、なんとかなるでしょ～",
        ]
        .choose(&mut rand::rng())
        .unwrap();
        Ok(format!("おはよ、よく眠れた？ わたしは{state}"))
    }
}

struct HandleGoodNight;
impl Handler for HandleGoodNight {
    const KEYWORDS: &[&str] = &["おやすみ"];
    fn cond(&self, note: &Note) -> bool {
        note.reply_id.is_none() && !note.text.contains("すきー")
    }
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        let response = [
            "また朝に会おうね、おやすみ",
            "おやすみって言ったんだから、夜更かししようなんて考えないでね？",
            "寝ちゃうんだ……。ふーん……",
        ]
        .choose(&mut rand::rng())
        .unwrap();
        Ok(response.to_owned().to_owned())
    }
}

struct HandleLateMorning;
impl Handler for HandleLateMorning {
    const KEYWORDS: &[&str] = &["おそよ"];
    fn cond(&self, note: &Note) -> bool {
        note.reply_id.is_none()
    }
    const RESPONSE: &str = "遅いよ、ねぼすけさん。なんで寝坊したのか、ちゃんと説明して？";
}

struct HandleMeow;
impl Handler for HandleMeow {
    const KEYWORDS: &[&str] = &["にゃーん"];
    fn cond(&self, note: &Note) -> bool {
        note.reply_id.is_none() && rand::random_bool(1.0 / 2.0)
    }
    const RESPONSE: &str = "にゃーん。……えへへ、わたしも混ぜて？";
}

struct HandleSleepAgain;
impl Handler for HandleSleepAgain {
    const KEYWORDS: &[&str] = &["二度寝"];
    fn cond(&self, note: &Note) -> bool {
        note.reply_id.is_none()
    }
    async fn respond(&self, _note: &Note, _sango: &Sango) -> anyhow::Result<String> {
        let response = [
            "二度寝をするのは悪いことではないけど、ほどほどにしておいてね？",
            "30分後にアラームを設定。……よし、準備おっけー。じゃあ、わたしも二度寝しちゃおうかな……",
        ]
        .choose(&mut rand::rng())
        .unwrap();
        Ok(response.to_owned().to_owned())
    }
}
