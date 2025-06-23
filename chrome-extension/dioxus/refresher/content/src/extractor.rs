use anyhow::anyhow;
use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, NodeList};

pub(super) fn extract_basic() -> anyhow::Result<String> {
	let document = document();

	let body = document.body().ok_or(anyhow!("No body found"))?;
	let noise_selectors = [
		"script",
		"style",
		"noscript",
		"iframe",
		"header",
		"footer",
		"nav",
		"aside",
		".sidebar",
		"#sidebar",
		".ad",
		".ads",
		".advertisement",
		".cookie-banner",
		"#cookie-banner",
		".social-share",
		".share-buttons",
		"#comments",
		".comments-section",
	];

	let cloned_body = body
		.clone_node_with_deep(true)
		.map_err(|_| anyhow!("Cloning error"))?;
	let cloned_body = cloned_body
		.dyn_into::<HtmlElement>()
		.map_err(|_| anyhow!("Cast error"))?;

	for selector in &noise_selectors {
		remove_elements(&document, &cloned_body, selector);
	}

	Ok(clean_content(
		&cloned_body.text_content().unwrap_or_default(),
	))
}

fn remove_elements(document: &Document, parent: &Element, selector: &str) {
	if let Ok(elements) = document.query_selector_all(selector) {
		remove_nodes(parent, elements);
	}
}

fn remove_nodes(parent: &Element, nodes: NodeList) {
	for i in 0..nodes.length() {
		if let Some(node) = nodes.get(i) {
			let _ = parent.remove_child(&node);
		}
	}
}

fn clean_content(content: &str) -> String {
	content
		.split_whitespace()
		.collect::<Vec<_>>()
		.join(" ")
		.trim()
		.to_string()
}
