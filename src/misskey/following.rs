// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use serde::Serialize;

use crate::misskey::ApiRequest;

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

impl ApiRequest for CreateFollowing {
    const ENDPOINT: &str = "/api/following/create";
    type Return = ();
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFollowing {
    user_id: String,
}

impl DeleteFollowing {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_owned(),
        }
    }
}

impl ApiRequest for DeleteFollowing {
    const ENDPOINT: &str = "/api/following/delete";
    type Return = ();
}
