// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowUser {
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub user_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl ShowUser {
    pub fn by_user_id(user_id: &str) -> Self {
        Self {
            user_id: Some(user_id.to_owned()),
            ..Default::default()
        }
    }

    // Unused
    /*
    pub fn by_username(username: &str, host: Option<&str>) -> Self {
        Self {
            username: Some(username.to_owned()),
            host: host.map(|ch| ch.to_owned()),
            ..Default::default()
        }
    }
    */
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserOnlineStatus {
    Unknown,
    Online,
    Active,
    Offline,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub host: Option<String>,
    // pub avatar_url: Option<String>, // Unused
    // pub avatar_blurhash: Option<String>, // Unused
    // pub avatar_decorations: todo!(), // めんどくさいしたぶん使わない
    pub is_bot: bool,
    // pub is_cat: bool, // Unused
    // pub emojis: todo!(), // めんどくさいしたぶん使わない
    // pub online_status: UserOnlineStatus, // Unused
    // pub badge_roles: todo!(), // めんどくさいしたぶん使わない
}

impl User {
    pub fn mention(&self) -> String {
        self.host.as_ref().map_or_else(
            || format!("@{}", self.username),
            |host| format!("@{}@{host}", self.username),
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailed {
    pub is_following: bool,
    pub is_followed: bool,
    // いまのところフォロー関係以外に興味なし
}
