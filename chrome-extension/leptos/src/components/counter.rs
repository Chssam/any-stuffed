use leptos::{ev, html::*, prelude::*};

#[component]
pub fn SimpleCounter(initial_value: i32, step: i32) -> impl IntoView {
	let (value, set_value) = signal(initial_value);

	let clear = move |_| set_value.set(initial_value);
	let decrement = move |_| set_value.update(|v| *v -= step);
	let increment = move |_| set_value.update(|v| *v += step);

	div().child((
		button().on(ev::click, clear).child("Clear"),
		button().on(ev::click, decrement).child(("-", step)),
		span().child(("Value: ", value, "!")),
		button().on(ev::click, increment).child(("+", step)),
	))
}
