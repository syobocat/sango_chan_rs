// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use serde::{Deserialize, Serialize};

use crate::misskey::users::User;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NoteVisibility {
    Public,
    Home,
    Followers,
    Specified,
}

// Unused
/*
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReactionAcceptance {
    LikeOnly,
    LikeOnlyForRemote,
    NonSensitiveOnly,
    NonSensitiveOnlyForLocalLikeOnlyForRemote,
}
*/

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePoll {
    pub choices: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_after: Option<u32>,
}

// Unused
/*
#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    pub choices: Vec<String>,
    pub multiple: bool,
    pub expires_at: Option<String>,
}
*/

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<NoteVisibility>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub visible_user_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub reaction_acceptance: Option<ReactionAcceptance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_extract_mentions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_extract_hashtags: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_extract_emojis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub media_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreatePoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<i32>,
}

impl CreateNote {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            ..Default::default()
        }
    }

    pub fn text(&self, text: &str) -> Self {
        Self {
            text: text.to_owned(),
            ..self.clone()
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    // pub created_at: String, // Unused
    // pub deleted_at: Option<String>, // Unused
    pub text: String,
    // pub cw: Option<String>, // Unused
    pub user_id: String,
    pub user: User,
    pub reply_id: Option<String>,
    // pub renote_id: Option<String>, // Unused
    // pub reply: todo!(), // めんどくさいしたぶん使わない
    // pub renote: todo!(), // めんどくさいしたぶん使わない
    // pub is_hidden: bool, // 謎
    pub visibility: NoteVisibility,
    #[serde(default)]
    pub mentions: Vec<String>,
    // pub visible_user_ids: Vec<String>, // 謎
    // pub file_ids: Vec<String>, // Unused
    // pub files: todo!(), // めんどくさいしたぶん使わない
    // pub tags: Vec<String>, // 謎
    // pub poll: Option<Poll>, // Unused
    // pub emojis: todo!(), // めんどくさいしたぶん使わない
    // pub channel_id: Option<String>, // Unused
    // pub channel: todo!(), // めんどくさいしたぶん使わない
}
