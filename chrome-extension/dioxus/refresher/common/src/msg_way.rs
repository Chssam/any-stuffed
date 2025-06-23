use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
	Content(ContentMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentMessage {
	PageLoaded(String),
	RemovedAds(String),
	Tell(String),
}
