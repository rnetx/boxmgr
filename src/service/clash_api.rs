use std::{fmt::Debug, net::SocketAddr};

use futures_util::{Future, StreamExt};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_util::sync::CancellationToken;

pub(crate) enum ClashAPIType {
    Traffic,
    Speed,
    Memory,
}

impl ClashAPIType {
    pub(crate) fn create_url(&self, listen: SocketAddr, secret: Option<String>) -> String {
        let mut url = format!("ws://{}", listen);
        match self {
            Self::Traffic => url.push_str("/connections"),
            Self::Speed => url.push_str("/traffic"),
            Self::Memory => url.push_str("/memory"),
        }
        if let Some(secret) = secret {
            format!("{}?token={}", url, secret)
        } else {
            url
        }
    }

    pub(crate) async fn handle<P, T, F, Fut>(
        &self,
        listen: SocketAddr,
        secret: Option<String>,
        token: CancellationToken,
        params: P,
        callback: F,
    ) where
        P: Clone + Send,
        T: serde::de::DeserializeOwned + Debug + Send,
        F: Fn(P, T) -> Fut,
        Fut: Future<Output = ()>,
    {
        let req = self
            .create_url(listen, secret)
            .into_client_request()
            .unwrap();
        let mut stream = match tokio::select! {
          _ = token.cancelled() => return,
          res = tokio_tungstenite::connect_async(req) => res,
        } {
            Ok((stream, _)) => stream,
            Err(e) => {
                log::error!("failed to connect to clash api: {}", e);
                return;
            }
        };
        loop {
            tokio::select! {
              _ = token.cancelled() => break,
              res = stream.next() => {
                match res {
                  Some(Ok(msg)) => {
                    match msg {
                      tokio_tungstenite::tungstenite::Message::Text(text) => {
                        if let Ok(data) = serde_json::from_str::<'_, T>(&text) {
                          tokio::select! {
                            _ = token.cancelled() => break,
                            _ = callback(params.clone(), data) => {}
                          }
                        }
                      }
                      _ => {}
                    }
                  }
                  Some(Err(e)) => {
                    log::error!("failed to receive message from clash api: {}", e);
                    break;
                  }
                  None => {}
                }
              }
            }
        }
        let _ = stream.close(None).await;
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ClashAPITrafficResult {
    pub(crate) connections: Option<Vec<serde_json::Value>>,
    #[serde(rename = "downloadTotal")]
    pub(crate) download_traffic: u64,
    #[serde(rename = "uploadTotal")]
    pub(crate) upload_traffic: u64,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ClashAPISpeedResult {
    #[serde(rename = "up")]
    pub(crate) upload_speed: u64,
    #[serde(rename = "down")]
    pub(crate) download_speed: u64,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ClashAPIMemoryResult {
    #[serde(rename = "inuse")]
    pub(crate) memory: u64,
}
