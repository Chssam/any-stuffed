use wasm_bindgen::prelude::*;

pub mod macroing;
mod popup;

use popup::*;
// use web_sys::WorkerGlobalScope;

#[wasm_bindgen]
pub async fn run_in_background() -> Result<(), JsValue> {
	log!("Hi From Rust Background Task");
	Ok(())
}

#[wasm_bindgen]
pub async fn run_in_foreground() -> Result<(), JsValue> {
	let window = web_sys::window().expect("Should have a window in this context");
	let document = window.document().expect("Window should have a document");
	let body = document.body().expect("Document should have a body");

	// let worker = match get_browser().await {
	// 	Ok(work) => work,
	// 	Err(err) => {
	// 		log!("{}", err);
	// 		return Err(JsValue::from_str(err));
	// 	},
	// };

	// test_click(&document, &worker);

	remove_loading(&document);
	setup_popup(&document, &body)?;
	receive_greet(&document);

	Ok(())
}

pub fn refreshing() {
	let window = web_sys::window().unwrap();
	if let Err(err) = window.location().reload() {
		web_sys::console::error_1(&err);
		log!("Fail to reload");
	}
	log!("Reload");
}

// async fn get_browser() -> std::result::Result<WorkerGlobalScope, &'static str> {
// 	let re_glob_scope = js_sys::global().dyn_into::<WorkerGlobalScope>();

// 	if let Ok(worker) = re_glob_scope {
// 		return Ok(worker);
// 	};
// 	Err("Missing browser")
// }
