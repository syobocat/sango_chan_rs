// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use anyhow::Context;
use reqwest::Client;
use serde_json::json;

pub mod following;
pub mod notes;
pub mod users;

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

    pub async fn notes_create(&self, params: notes::CreateNote) -> anyhow::Result<()> {
        let host = &self.host;
        let resp = self
            .client
            .post(format!("https://{host}/api/notes/create"))
            .bearer_auth(&self.token)
            .json(&params)
            .send()
            .await
            .context("Failed to create a note")?;
        resp.error_for_status()?;
        Ok(())
    }

    pub async fn users_show(&self, params: users::ShowUser) -> anyhow::Result<users::UserDetailed> {
        let host = &self.host;
        let resp = self
            .client
            .post(format!("https://{host}/api/users/show"))
            .bearer_auth(&self.token)
            .json(&params)
            .send()
            .await
            .context("Failed to get a user")?;
        let user = resp.json().await.context("Failed to parse the response")?;
        Ok(user)
    }

    pub async fn following_create(&self, params: following::CreateFollowing) -> anyhow::Result<()> {
        let host = &self.host;
        let resp = self
            .client
            .post(format!("https://{host}/api/following/create"))
            .bearer_auth(&self.token)
            .json(&params)
            .send()
            .await
            .context("Failed to create a following")?;
        resp.error_for_status()?;
        Ok(())
    }

    pub async fn following_delete(&self, params: following::DeleteFollowing) -> anyhow::Result<()> {
        let host = &self.host;
        let resp = self
            .client
            .post(format!("https://{host}/api/following/delete"))
            .bearer_auth(&self.token)
            .json(&params)
            .send()
            .await
            .context("Failed to delete a following")?;
        resp.error_for_status()?;
        Ok(())
    }
}
