use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExtractionMode {
	Basic,
	HideId,
	HideRelate,
	AddHideId,
	Store,
}

#[derive(Serialize, Deserialize)]
pub struct ExtractRequest {
	pub mode: ExtractionMode,
	pub bonus: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExtractResponse {
	pub content: String,
}
