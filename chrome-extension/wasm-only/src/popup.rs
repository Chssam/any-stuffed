use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, HtmlInputElement};

pub(super) fn setup_popup(document: &Document, body: &HtmlElement) -> Result<(), JsValue> {
	let input_box = document.create_element("input")?;
	input_box.set_id("name");
	input_box.set_attribute("placeholder", "Enter your name")?;

	let greet_box = document.create_element("button")?;
	greet_box.set_id("greet");
	greet_box.set_attribute("type", "text")?;
	greet_box.set_attribute("value", "Greet")?;
	greet_box.set_attribute("name", "greet")?;
	greet_box.set_inner_html("Greet");

	let output_box = document.create_element("div")?;
	output_box.set_id("output");
	output_box.set_attribute("type", "text")?;
	output_box.set_attribute("class", "output")?;

	let test_box = document.create_element("button")?;
	test_box.set_id("tester");
	test_box.set_attribute("type", "text")?;
	test_box.set_attribute("value", "Tester")?;
	test_box.set_attribute("name", "tester")?;
	test_box.set_inner_html("Tester");

	let select_box = document.create_element("select")?;

	let op_1_box = document.create_element("option")?;
	op_1_box.set_inner_html("Option One");

	let op_2_box = document.create_element("option")?;
	op_2_box.set_inner_html("Option Two");

	select_box.append_with_node_2(&op_1_box, &op_2_box)?;

	body.append_with_node_5(&input_box, &greet_box, &output_box, &test_box, &select_box)?;

	Ok(())
}

pub(super) fn receive_greet(document: &Document) {
	let name = document
		.get_element_by_id("name")
		.expect("Should have #name on the popup");

	let output = document
		.get_element_by_id("output")
		.expect("Should have #output on the popup");

	let click_greet = Closure::<dyn FnMut()>::new(move || {
		let mut input_name = name
			.dyn_ref::<HtmlInputElement>()
			.expect("#greet should be `HtmlElement`")
			.value();

		if input_name.is_empty() {
			input_name = "Rust Maker".to_string();
		}

		let out_greet = format!("Hello {}. You Successful processed.", input_name);

		output.set_inner_html(&out_greet);
	});

	document
		.get_element_by_id("greet")
		.expect("Should have #greet on the popup")
		.dyn_ref::<HtmlElement>()
		.expect("#greet should be `HtmlElement`")
		.set_onclick(Some(click_greet.as_ref().unchecked_ref()));

	click_greet.forget();
}

pub(super) fn remove_loading(document: &Document) {
	let loading = document
		.get_element_by_id("loading")
		.expect("Should have #loading on the popup");
	loading.remove();
}

// pub(super) fn test_click(document: &Document, worker: &WorkerGlobalScope) {
// 	let work_loc = worker.location();
// 	let href = work_loc.href();
// 	let orig = work_loc.origin();
// 	let host = work_loc.host();
// 	let host_name = work_loc.hostname();
// 	let click_test = Closure::<dyn FnMut()>::new(move || {
// 		let longer = format!("{href} ; {orig} ; {host} ; {host_name}");
// 		log!("{}", longer);
// 	});

// 	document
// 		.get_element_by_id("tester")
// 		.expect("Should have #tester on the popup")
// 		.dyn_ref::<HtmlElement>()
// 		.expect("#tester should be `HtmlElement`")
// 		.set_onclick(Some(click_test.as_ref().unchecked_ref()));

// 	click_test.forget();
// }
