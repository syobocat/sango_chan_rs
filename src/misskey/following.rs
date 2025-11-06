// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFollowing {
    user_id: String,
}

impl CreateFollowing {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_owned(),
        }
    }
}

pub type DeleteFollowing = CreateFollowing;
