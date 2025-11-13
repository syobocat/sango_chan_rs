// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use rand::seq::IndexedRandom;

use crate::{Sango, misskey::notes::Note};

pub async fn on_note(note: Note, sango: &Sango) -> anyhow::Result<()> {
    let text = &note.text;

    let response = if text.contains("つらい") || text.contains("つらすぎ") {
        "つらいときは、甘えてもいいんだよ？"
    } else if text.contains("疲れた")
        || text.contains("つかれた")
        || text.contains("疲れてる")
        || text.contains("つかれてる")
        || text.contains("疲れている")
        || text.contains("つかれている")
    {
        "ひとやすみ、する？ それとも、わたしが癒してあげよっか？"
    } else if text.contains("出勤") {
        gowork()
    } else if text.contains("退勤") {
        "お仕事終わったの？ お疲れさま～。 ……わたしの癒し、必要かな？ 必要なら、いつでも言ってね"
    } else if text.contains("ぬるぽ") && rand::random_bool(1.0 / 3.0) {
        "ガッ" // 本家では絵文字使ってる
    } else if text.contains("さんごちゃん")
        && !text.trim_end().ends_with("さんごちゃん") // 「さんごちゃん」以降に文字がある場合のみ
        && rand::random_bool(1.0 / 3.0)
    {
        let name = sango.savedata.read().await.get_displayname(&note.user);
        &format!("呼んだ？ {name}さん")
    } else if note.reply_id.is_none() {
        if text.contains("眠い")
            || text.contains("眠たい")
            || text.contains("ねむ") && !text.contains("くない")
        {
            "なるほど、眠いんだね。……我慢はよくないよ？ 欲には素直にならないと"
        } else if text.contains("おはよ") {
            &format!("おはよ、よく眠れた？ わたしは{}", morning())
        } else if text.contains("おやすみ") && !text.contains("すきー") {
            goodnight()
        } else if text.contains("おそよ") {
            "遅いよ、ねぼすけさん。なんで寝坊したのか、ちゃんと説明して？"
        } else if text.contains("にゃーん") && rand::random_bool(1.0 / 2.0) {
            "にゃーん。……えへへ、わたしも混ぜて？"
        } else if text.contains("二度寝") {
            twotimesleep()
        } else {
            return Ok(());
        }
    } else {
        return Ok(());
    };

    sango.client.notes_create(note.reply(response)).await?;

    Ok(())
}

fn morning() -> &'static str {
    [
        "よく眠れたよ～。元気いーっぱい",
        "あんまり寝れなかったかな……。まぁ、なんとかなるでしょ～",
    ]
    .choose(&mut rand::rng())
    .unwrap()
}

fn goodnight() -> &'static str {
    [
        "また朝に会おうね、おやすみ",
        "おやすみって言ったんだから、夜更かししようなんて考えないでね？",
        "寝ちゃうんだ……。ふーん……",
    ]
    .choose(&mut rand::rng())
    .unwrap()
}

fn gowork() -> &'static str {
    [
        "お仕事、頑張ってきてね。わたし、帰ってくるの、待ってるから……",
        "お仕事は大事だけど、あんまり無理はしないでね？",
        "お仕事とわたし、どっちが大事なんだろう……。まぁ、わたしにはロイちゃんがいるから、いい……のかな？\n……あっ、ち、違う！ これは違くて…！ なんでもないから……！",
    ].choose(&mut rand::rng()).unwrap()
}

fn twotimesleep() -> &'static str {
    [
        "二度寝をするのは悪いことではないけど、ほどほどにしておいてね？",
        "30分後にアラームを設定。……よし、準備おっけー。じゃあ、わたしも二度寝しちゃおうかな……",
    ]
    .choose(&mut rand::rng())
    .unwrap()
}
