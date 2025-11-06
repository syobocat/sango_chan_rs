// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use anyhow::Context;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Bytes, Message, Utf8Bytes},
};

pub struct MisskeyWebsocket(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl MisskeyWebsocket {
    pub async fn new(host: &str, token: &str) -> anyhow::Result<Self> {
        let mut ws = Self::connect(host, token).await?;
        ws.subscribe().await?;
        Ok(ws)
    }

    async fn connect(host: &str, token: &str) -> anyhow::Result<Self> {
        let (ws, _) = tokio_tungstenite::connect_async(format!("wss://{host}/streaming?i={token}"))
            .await
            .context("Failed to connect to ws")?;
        Ok(Self(ws))
    }

    async fn subscribe(&mut self) -> anyhow::Result<()> {
        let main_req = json!({
            "type": "connect",
            "body": {
                "channel": "main",
                "id": "main_channel",
            },
        });
        let home_req = json!({
            "type": "connect",
            "body": {
                "channel": "homeTimeline",
                "id": "home_channel",
            },
        });

        self.0
            .feed(Message::Text(Utf8Bytes::from(main_req.to_string())))
            .await
            .context("Failed to send to ws")?;
        self.0
            .feed(Message::Text(Utf8Bytes::from(home_req.to_string())))
            .await
            .context("Failed to send to ws")?;
        self.0.flush().await.context("Failed to send to ws")?;
        Ok(())
    }

    async fn pong(&mut self, bytes: Bytes) -> anyhow::Result<()> {
        self.0
            .send(Message::Pong(bytes))
            .await
            .context("Failed to send pong")?;
        Ok(())
    }

    pub async fn next(&mut self) -> anyhow::Result<EventBody> {
        loop {
            let next = self.0.next().await.context("Connection terminated")?;
            let message = next.context("Failed to get the next value")?;
            match message {
                Message::Ping(bytes) => {
                    self.pong(bytes).await?;
                }
                Message::Text(bytes) => {
                    let text = bytes.as_str();
                    let Ok(json) = serde_json::from_str::<WebsocketEvent>(text) else {
                        log::debug!("Received data is in unknown format: {text}");
                        continue;
                    };
                    return Ok(json.body);
                }
                _ => {}
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum EventType {
    Channel,
}

#[derive(Deserialize)]
struct WebsocketEvent {
    #[serde(rename = "type")]
    pub _event_type: EventType, // typeがchannelかどうか判断するのにしか使わない
    body: EventBody,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventBodyType {
    Note,
    Notification, // Unused
    Mention,
    Reply,  // Unused
    Renote, // Unused
    Follow, // Unused
    Followed,
    Unfollow,                    // Unused
    MessagingMessage,            // Unused
    ReadAllNotifications,        // Unused
    UnreadNotification,          // Unused
    UnreadMention,               // Unused
    ReadAllUnreadMentions,       // Unused
    UnreadSpecifiedNote,         // Unused
    ReadAllUnreadSpecifiedNotes, // Unused
    UnreadMessagingMessage,      // Unused
    ReadAllMessagingMessages,    // Unused
}

#[derive(Deserialize)]
pub struct EventBody {
    // pub id: String, // Unused
    #[serde(rename = "type")]
    pub event_type: EventBodyType,
    pub body: serde_json::Value,
}
