use leptos::{
	ev,
	html::{self, *},
	prelude::*,
};

#[component]
pub fn Greet() -> impl IntoView {
	let (name, set_name) = signal("Hello World".to_string());

	let input_element: NodeRef<html::Input> = NodeRef::new();

	let on_submit = move |ev: ev::SubmitEvent| {
		ev.prevent_default();

		let value = input_element
			.get()
			.expect("<input> should be mounted")
			.value();
		set_name.set(value);
	};

	div().child((
		form().on(ev::submit, on_submit).child((
			input()
				.id("name")
				.r#type("text")
				.node_ref(input_element)
				.placeholder("Insert Name"),
			input().r#type("submit").value("Submit"),
		)),
		p().child(("Name is: ", name)),
	))
}
