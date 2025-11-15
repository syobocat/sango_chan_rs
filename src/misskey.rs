// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use anyhow::Context;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

pub mod following;
pub mod notes;
pub mod users;

pub trait ApiRequest {
    const ENDPOINT: &str;
    type Return;
}

pub struct MisskeyClient {
    client: Client,
    host: String,
    token: String,
}

impl MisskeyClient {
    pub fn new(host: &str, token: &str) -> Self {
        let client = Client::new();
        let host = host.to_owned();
        let token = token.to_owned();
        Self {
            client,
            host,
            token,
        }
    }

    pub async fn get_id_self(&self) -> anyhow::Result<String> {
        let host = &self.host;
        let resp = self
            .client
            .post(format!("https://{host}/api/i"))
            .bearer_auth(&self.token)
            .json(&json!({}))
            .send()
            .await
            .context("Failed to authorize")?;
        let i: users::User = resp.json().await.context("Failed to authorize")?;
        Ok(i.id)
    }

    pub async fn request<Req>(&self, params: Req) -> anyhow::Result<Req::Return>
    where
        Req: Serialize + ApiRequest,
        Req::Return: DeserializeOwned,
    {
        let host = &self.host;
        let resp = self
            .client
            .post(format!("https://{host}{}", Req::ENDPOINT))
            .bearer_auth(&self.token)
            .json(&params)
            .send()
            .await
            .context("Failed to send request")?;
        let resp = resp.error_for_status()?;
        let ret = resp.json().await.context("Failed to parse the response")?;
        Ok(ret)
    }
}
