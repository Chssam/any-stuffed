use anyhow::anyhow;
use common::extraction::*;
use gloo::utils::{format::JsValueSerdeExt, window};
use js_sys::{Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub async fn extract(mode: ExtractionMode, bonus: String) -> Result<String, anyhow::Error> {
	let window = window();

	let chrome = Reflect::get(&window, &JsValue::from_str("chrome"))
		.map_err(|_| anyhow!("Chrome API not available"))?;
	let tabs = Reflect::get(&chrome, &JsValue::from_str("tabs"))
		.map_err(|_| anyhow!("Tabs API not available"))?;

	let query_fn = Reflect::get(&tabs, &JsValue::from_str("query"))
		.map_err(|_| anyhow!("Query method not available"))?
		.dyn_into::<js_sys::Function>()
		.map_err(|_| anyhow!("Failed to cast query to function"))?;

	let query_obj = Object::new();
	Reflect::set(
		&query_obj,
		&JsValue::from_str("active"),
		&JsValue::from_bool(true),
	)
	.map_err(|_| anyhow!("Failed to set active property"))?;
	Reflect::set(
		&query_obj,
		&JsValue::from_str("currentWindow"),
		&JsValue::from_bool(true),
	)
	.map_err(|_| anyhow!("Faailed to set currentWindow property"))?;

	let tabs_promise = query_fn
		.call1(&tabs, &query_obj)
		.map_err(|_| anyhow!("Failed to query tabs"))?;

	// wait for active tab query complete
	let tabs_promise = tabs_promise
		.dyn_into::<js_sys::Promise>()
		.map_err(|e| anyhow!("Failed to convert to promise: {e:?}"))?;
	let tabs_result = JsFuture::from(tabs_promise)
		.await
		.map_err(|_| anyhow!("Failed to get tab ID"))?;

	let tabs_array = js_sys::Array::from(&tabs_result);
	if tabs_array.length() == 0 {
		return Err(anyhow!("No active tab found"));
	}

	let active_tab = tabs_array.get(0);
	let tab_id = Reflect::get(&active_tab, &JsValue::from_str("id"))
		.map_err(|_| anyhow!("Failed to get tab ID"))?;

	let request = ExtractRequest { mode, bonus };

	let request_js =
		JsValue::from_serde(&request).map_err(|_| anyhow!("Failed to serialize request"))?;

	// chrome.tabs.sendMessage
	let send_msg_fn = Reflect::get(&tabs, &JsValue::from_str("sendMessage"))
		.map_err(|_| anyhow!("sendMessage method not availaable"))?
		.dyn_into::<js_sys::Function>()
		.map_err(|_| anyhow!("Failed to cast sendMessage to function"))?;

	// msg to content script
	let tab_id_f64 = tab_id
		.as_f64()
		.ok_or(anyhow!("Failed to convert tab ID to f64"))?;
	let send_promise = send_msg_fn
		.call2(&tabs, &JsValue::from_f64(tab_id_f64), &request_js)
		.map_err(|_| anyhow!("Failed to send message to tab"))?
		.dyn_into::<js_sys::Promise>()
		.map_err(|_| anyhow!("Failed to convert sendMessage result to promise"))?;

	let response_js = JsFuture::from(send_promise)
		.await
		.map_err(|e| anyhow!("Failed to get response from content script: {:?}", e))?;

	let response: ExtractResponse = response_js
		.into_serde()
		.map_err(|e| anyhow!("Failed to deserialize response: {:?}", e))?;

	Ok(response.content)
}
