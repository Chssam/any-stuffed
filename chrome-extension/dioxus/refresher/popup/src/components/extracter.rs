use common::extraction::ExtractionMode;
use dioxus::prelude::*;
use gloo::console::log;

use crate::components::extractor::extract;

#[component]
pub fn Extract_button() -> Element {
	let mut extraction_mode = use_signal(|| ExtractionMode::Basic);
	let mut is_extracting = use_signal(|| false);
	let mut bonus_input = use_signal(|| String::new());

	let extract_content = move |_| {
		is_extracting.set(true);

		spawn(async move {
			match extract(extraction_mode(), bonus_input()).await {
				Ok(result) => {
					log!(&result);
				},
				Err(e) => {
					log!(e.to_string());
				},
			}
			is_extracting.set(false);
		});
	};

	rsx! {
		input {
			r#type: "text",
			oninput: move |event| bonus_input.set(event.value())
		}
		button {
			disabled: is_extracting(),
			onclick: extract_content,
			if is_extracting() {
				"Waiting signal..."
			} else {
				"Send Signal"
			}
		}
		button {
			disabled: is_extracting(),
			onclick: move |_| {
				let mode = match extraction_mode() {
						ExtractionMode::Basic=>ExtractionMode::HideId,
						ExtractionMode::HideId =>ExtractionMode::HideRelate,
						ExtractionMode::HideRelate =>ExtractionMode::AddHideId,
						ExtractionMode::AddHideId =>ExtractionMode::Store,
						ExtractionMode::Store =>ExtractionMode::Basic,
					};
				extraction_mode.set(mode);
			},
			"Mode: {extraction_mode():?}"
		}
	}
}
