use components::{extracter::Extract_button, global::FastTest};
use dioxus::{
	logger::{init, tracing},
	prelude::*,
	web::{Config, launch},
};
use gloo::{console::info, utils::document};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

mod components;

#[wasm_bindgen(start)]
pub fn run_popup() {
	console_error_panic_hook::set_once();
	init(tracing::Level::INFO).expect("failed to init logger");
	setup_popup_ui();
	info!("Initialized Popup UI successfully");
}

pub fn setup_popup_ui() {
	let document = document();
	let got_main = document.get_element_by_id("main");
	if got_main.is_none() {
		return;
	}
	remove_loading();

	launch::launch_cfg(App, Config::default());
}

#[component]
pub fn App() -> Element {
	rsx! {
		// div { "Hello World" }
		// Greet {}
		// Counter {}
		FastTest {}
		Extract_button {}
	}
}

pub fn remove_loading() {
	let Some(load_box) = document().get_element_by_id("loading") else {
		return;
	};

	load_box
		.dyn_ref::<HtmlElement>()
		.expect("#load_box should be html element")
		.remove();
}
