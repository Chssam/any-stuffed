use common::msg_way::{ContentMessage, Message};
use gloo::utils::window;
use js_sys::Reflect;
use wasm_bindgen::{JsCast, JsValue};

pub async fn send_message(message: ContentMessage) -> Result<(), JsValue> {
	let msg = Message::Content(message);

	let window = window();
	let msg_js = serde_wasm_bindgen::to_value(&msg)?;

	let browser = Reflect::get(&window, &JsValue::from_str("chrome"))?;
	let runtime = Reflect::get(&browser, &JsValue::from_str("runtime"))?;
	let send_msg = Reflect::get(&runtime, &JsValue::from_str("sendMessage"))?;

	Reflect::apply(
		&send_msg.dyn_into::<js_sys::Function>()?,
		&runtime,
		&js_sys::Array::of1(&msg_js),
	)?;

	Ok(())
}
