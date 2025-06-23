use std::rc::{Rc, Weak};

use common::{
	extraction::{ExtractRequest, ExtractResponse, ExtractionMode},
	msg_way::ContentMessage,
};
use extractor::extract_basic;
use gloo::{
	console::{error, log},
	utils::{document, format::JsValueSerdeExt, window},
};
use js_sys::{Function, Reflect};
use shared::send_message;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{
	storing::{Store, get_store},
	*,
};

pub fn setup_when_loaded(rc_store: Rc<Store>) -> Result<(), JsValue> {
	let window = window();
	let document = document();

	let loaded = Closure::<dyn Fn()>::new(move || {
		spawn_local(async {
			if let Err(e) = hide_ads().await {
				let err = format!("Content script error: {e:?}");
				error!(err);
			}
		});

		let Some(query_video) = document.query_selector("video").ok().flatten() else {
			log!("No Video found");
			return;
		};

		let pause_closure = Closure::<dyn Fn()>::new(move || {
			call_pause();
		});

		let pause_marked = query_video
			.add_event_listener_with_callback("pause", pause_closure.as_ref().unchecked_ref());
		if let Err(e) = pause_marked {
			let err = format!("Fail to add pause: {e:?}");
			error!(err);
		}

		pause_closure.forget();

		let play_closure = Closure::<dyn Fn()>::new(move || {
			spawn_local(async {
				let telling = send_message(ContentMessage::Tell("Play".to_string())).await;
				if let Err(e) = telling {
					let err = format!("Fail To Tell: {e:?}");
					error!(err);
				}
			});
			log!("Some Play");
		});

		let play_marked = query_video
			.add_event_listener_with_callback("play", play_closure.as_ref().unchecked_ref());
		if let Err(e) = play_marked {
			let err = format!("Fail to add pause: {e:?}");
			error!(err);
		}

		play_closure.forget();

		log!("Created Listener for video");
	});

	window.add_event_listener_with_callback("load", loaded.as_ref().unchecked_ref())?;
	loaded.forget();
	Ok(())
}

pub fn setup_listener(rc_store: Rc<Store>) -> Result<(), JsValue> {
	let window = window();
	let browser = Reflect::get(&window, &JsValue::from_str("chrome"))?;
	let runtime = Reflect::get(&browser, &JsValue::from_str("runtime"))?;
	let on_message = Reflect::get(&runtime, &JsValue::from_str("onMessage"))
		.expect("Expect 'onMessage' property on runtime");
	let add_listener = Reflect::get(&on_message, &JsValue::from_str("addListener"))
		.expect("Expect 'addListener' property on onMessage");

	let closure = Closure::wrap(Box::new(
		move |message: JsValue, _sender: JsValue, response: Function| -> bool {
			send_the(message, response, rc_store.clone())
		},
	) as Box<dyn FnMut(JsValue, JsValue, Function) -> bool>);

	js_sys::Reflect::apply(
		&add_listener.dyn_into::<js_sys::Function>()?,
		&on_message,
		&js_sys::Array::of1(closure.as_ref().unchecked_ref()),
	)?;

	closure.forget();

	Ok(())
}

fn send_the(message: JsValue, response: Function, rc_store: Rc<Store>) -> bool {
	let op_extract = serde_wasm_bindgen::from_value::<ExtractRequest>(message);
	let Ok(extracted) = op_extract else {
		return false;
	};
	let content =
		extract_internal(extracted, rc_store).unwrap_or_else(|e| format!("Fail to process: {}", e));
	let request = ExtractResponse { content };
	let request_js = JsValue::from_serde(&request).expect("[send_the] serde should success");

	response
		.call1(&JsValue::from_str("String"), &request_js)
		.expect("should able to response");

	true
}

fn extract_internal(
	extracted: ExtractRequest,
	rc_store: Rc<Store>,
) -> Result<String, anyhow::Error> {
	log!(format!("Extraction mode: {:?}", extracted.mode));
	match extracted.mode {
		ExtractionMode::Basic => extract_basic(),
		ExtractionMode::HideId => hide_id(extracted.bonus),
		ExtractionMode::HideRelate => hide_related(extracted.bonus),
		ExtractionMode::AddHideId => add_hide_id(extracted.bonus, rc_store),
		ExtractionMode::Store => get_store(extracted.bonus),
	}
}

pub async fn check_content() -> Result<(), JsValue> {
	let document = document();
	let location = document
		.location()
		.expect("document should have a location");
	let url = location.href()?;

	send_message(ContentMessage::PageLoaded(url)).await?;

	let div_eleents = document.query_selector_all("div")?;
	let div_count = div_eleents.length();

	let the_msg = format!("div: {}", div_count);

	send_message(ContentMessage::Tell(the_msg)).await?;

	Ok(())
}
