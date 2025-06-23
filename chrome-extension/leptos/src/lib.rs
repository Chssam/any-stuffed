use components::{counter::SimpleCounter, global::Greet};
use leptos::{ev, prelude::*};
use log::info;
use wasm_bindgen::prelude::*;

mod components;

#[wasm_bindgen]
pub fn run_popup() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("Hello From Rust");
	mount_to_body(|| {
		view! {
			<SimpleCounter initial_value=5 step=5/>,
			<Greet/>
		}
	});
}
