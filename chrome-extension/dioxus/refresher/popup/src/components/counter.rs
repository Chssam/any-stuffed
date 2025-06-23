use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
	let int = 5;
	let step = 10;
	let mut count = use_signal(|| int);

	rsx! {
		h1 { "Nice Counter: {count}" }
		button { onclick: move |_| count += step, "Add: {step}" }
		button { onclick: move |_| count -= step, "Sub: {step}" }
		button { onclick: move |_| count -= 50, "Sub: 50" }
	}
}
