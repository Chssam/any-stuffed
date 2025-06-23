# Chrome extension but with Rust

I have try to find a way to make use of rust to create chrome-extension where it took enough 1 week to know how it works.

The setup are included in the reference part.

Best to go wasm-bindgen example first: [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

Both framework that I have experience with:
[Leptos](https://github.com/leptos-rs/leptos)
and
[Dioxus](https://github.com/dioxuslabs/dioxus)

## Reference

The way I follow up by order in success make it work: 

+ wasm-bindgen only - https://github.com/theberrigan/rust-wasm-chrome-ext

+ Leptos - Didn't find reference, but example would work if you try wasm-bindgen only first.
Later follow by Leptos's [example](https://github.com/leptos-rs/leptos/tree/main/examples)

+ Dioxus - https://github.com/Summit-Sailors/dioxus-browser-extension-builder

## Others

Contain 3 project that have been played around shortly.

More solid builds, check dioxus folder I created, it's mess but work.

The code in here are Windows tested code.

I recreate the Makefile.toml to work with Window. Original Linux from reference.

Dioxus Makefile.toml didn't work on my usecase, currently, but could refer the commands inside.

For Linux, the reference could help.

In other case, this show messing with the stuff.

