use dioxus::prelude::*;
use gloo::{console::log, utils::document};
use wasm_bindgen::JsValue;

#[component]
pub fn Greet() -> Element {
	let mut name = use_signal(|| "Bro".to_string());

	rsx! {
		form {
			onsubmit: move |event| {
				let input_name = event.values().get("name").unwrap().concat();
				name.set(input_name);
			},
			input {
				name: "name",
				r#type: "text",
				value: "{name}",
			}
			input { r#type: "submit" }
		}
		div { "Hi {name}" }
	}
}

#[component]
pub fn FastTest() -> Element {
	let check_fast = move |_| {
		if let Err(e) = check_url() {
			log!(e);
		}
	};

	rsx! {
		button { onclick: check_fast, "Fast Track Test" }
	}
}

fn check_url() -> Result<(), JsValue> {
	let document = document();
	let location = document
		.location()
		.expect("document should have a location");
	let url = location.href()?;
	log!(url);

	Ok(())
}
