use common::msg_way::{ContentMessage, Message};
use gloo::console::log;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn run_background() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();

	let global = js_sys::global();

	let chrome = Reflect::get(&global, &JsValue::from_str("chrome"))
		.expect("Expect 'chrome' in global object");
	let runtime = Reflect::get(&chrome, &JsValue::from_str("runtime"))
		.expect("Expect 'runtime' in chrome object");
	let on_message = Reflect::get(&runtime, &JsValue::from_str("onMessage"))
		.expect("Expect 'onMessage' property on runtime");
	let add_listener = Reflect::get(&on_message, &JsValue::from_str("addListener"))
		.expect("Expect 'addListener' property on onMessage");

	let closure = Closure::wrap(Box::new(move |message: JsValue| {
		spawn_local(async move {
			handle_message(message).await;
		});
	}) as Box<dyn FnMut(JsValue)>);

	js_sys::Reflect::apply(
		&add_listener.dyn_into::<js_sys::Function>()?,
		&on_message,
		&js_sys::Array::of1(closure.as_ref().unchecked_ref()),
	)?;

	closure.forget();

	log!("Background ready");

	Ok(())
}

async fn handle_message(event: JsValue) {
	let Ok(message) = serde_wasm_bindgen::from_value::<Message>(event) else {
		return;
	};
	match message {
		Message::Content(content_message) => handle_content_message(content_message).await,
	}
}

async fn handle_content_message(message: ContentMessage) {
	match message {
		ContentMessage::PageLoaded(mut url) => {
			url.insert_str(0, "[Page loaded] ");
			log!(url);
		},
		ContentMessage::RemovedAds(mut url) => {
			url.insert_str(0, "[Removed Ads] ");
			log!(url);
		},
		ContentMessage::Tell(mut msg) => {
			msg.insert_str(0, "[Content say] ");
			log!(msg);
		},
	}
}
