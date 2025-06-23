use std::{collections::HashMap, rc::Rc};

use anyhow::anyhow;
use common::msg_way::ContentMessage;
use gloo::{
	console::{error, info, log},
	utils::document,
};
use listener::{check_content, setup_listener, setup_when_loaded};
use shared::send_message;
use storing::{Store, setup_store};
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlMediaElement};

mod extractor;
mod listener;
mod shared;
mod storing;

#[wasm_bindgen(start)]
pub fn init_content() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();

	spawn_local(async {
		if let Err(e) = check_content().await {
			let err = format!("Content script error: {e:?}");
			error!(err);
		}
	});

	let rc_store = setup_store()?;
	setup_listener(rc_store.clone())?;
	setup_when_loaded(rc_store)?;

	Ok(())
}

pub fn call_pause() {
	let document = document();

	if let Some(element_video) = document
		.query_selector("video.video-stream.html5-main-video")
		.ok()
		.flatten()
	{
		let video = element_video
			.dyn_ref::<HtmlMediaElement>()
			.expect("Should be video");
		let stop_at = video.current_time();
		log!("Stop at: ", stop_at);
	};
}

pub async fn hide_ads() -> Result<(), JsValue> {
	let document = document();
	let location = document
		.location()
		.expect("document should have a location");
	let url = location.href()?;

	// Youtube
	if let Some(ads) = document.get_element_by_id("companion") {
		let msg = "Trying hide ads".to_string();
		send_message(ContentMessage::Tell(msg)).await?;

		let html = ads
			.dyn_ref::<HtmlElement>()
			.expect("Youtube's #companion should be html element");

		html.set_hidden(true);

		send_message(ContentMessage::Tell("Success".to_string())).await?;

		let is_success = html
			.set_attribute("display", "none")
			.map_err(|_| log!("Fail to hide"))
			.is_ok();
		if is_success {
			send_message(ContentMessage::RemovedAds(url)).await?;
		};
	}

	Ok(())
}

pub fn hide_id(bonus_input: String) -> anyhow::Result<String> {
	let document = document();

	let element = document
		.get_element_by_id(&bonus_input)
		.ok_or_else(|| anyhow!("Can't find the ID for: {}", bonus_input))?;

	let htmled = element
		.dyn_ref::<HtmlElement>()
		.ok_or_else(|| anyhow!("Not Html element for: {}", bonus_input))?;
	htmled.set_hidden(true);

	let out = format!("Hidden: {}", bonus_input);

	Ok(out)
}

pub fn hide_related(bonus_input: String) -> anyhow::Result<String> {
	let document = document();
	let related = document
		.query_selector_all(&bonus_input)
		.map_err(|_err| anyhow!("Unable to find query for: '{}'", bonus_input))?;

	info!(related);
	// 	let htmled = element.dyn_ref::<HtmlElement>().expect("It exist");
	// htmled.set_hidden(true);
	let out = format!("Hidden: {}", bonus_input);

	Ok(out)
}

pub fn add_hide_id(bonus_input: String, rc_store: Rc<Store>) -> anyhow::Result<String> {
	let document = document();
	let location = document
		.location()
		.expect("document should have a location");
	let raw_url = location
		.href()
		.map_err(|err| anyhow!("Location href fail: {:?}", err))?;
	let url = Url::parse(&raw_url).map_err(|err| anyhow!("Fail to parse url: {:?}", err))?;
	// rc_store.insert(url, bonus_input.clone());
	let out = format!("Added Hide ID: {}", bonus_input);
	Ok(out)
}

// fn hide_elements(document: &Document, parent: &Element, selector: &str) {
// 	if let Ok(elements) = document.query_selector_all(selector) {
// 		hide_nodes(parent, elements);
// 	}
// }

// fn hide_nodes(parent: &Element, nodes: NodeList) {
// 	for i in 0..nodes.length() {
// 		if let Some(node) = nodes.get(i) {
// 			let _ = parent.remove_child(&node);
// 		}
// 	}
// }
