use std::{
	collections::{HashMap, HashSet},
	rc::Rc,
};

use gloo::{console::info, utils::window};
use js_sys::JSON;
use serde::{Deserialize, Serialize};
use url::Url;
use wasm_bindgen::JsValue;

/// URL : IDs
#[derive(Default, Serialize, Deserialize)]
pub struct BlockedList(pub HashMap<String, HashSet<String>>);

pub struct Store {
	local_storage: web_sys::Storage,
	data: BlockedList,
	name: String,
}

impl Store {
	pub fn new(name: &str) -> Result<Store, JsValue> {
		let window = window();
		let local_storage = window
			.local_storage()?
			.ok_or_else(|| JsValue::from_str("No local storage"))?;
		let mut store = Store {
			local_storage,
			data: BlockedList::default(),
			name: String::from(name),
		};
		store.fetch_local_storage()?;
		Ok(store)
	}

	fn fetch_local_storage(&mut self) -> Result<(), JsValue> {
		let mut blocked_list = BlockedList::default();
		// If have an existing cached value, return early.
		if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
			let data = JSON::parse(&value)?;
			let list_block = serde_wasm_bindgen::from_value::<BlockedList>(data)?;
			blocked_list = list_block;
		}
		self.data = blocked_list;
		Ok(())
	}

	/// Write the local to localStorage.
	fn sync_local_storage(&mut self) {
		let Ok(mapped) =
			serde_wasm_bindgen::to_value(&self.data).map_err(|err| info!("Failed serialize:", err))
		else {
			return;
		};

		if let Ok(storage_string) = JSON::stringify(&mapped) {
			let storage_string: String = storage_string.into();
			self.local_storage
				.set_item(&self.name, &storage_string)
				.unwrap();
		}
	}

	pub fn insert(&mut self, url_in: Url, id: String) {
		let url = url_in.host_str().expect("Has Host name").to_string();
		self.data
			.0
			.entry(url)
			.and_modify(|set| {
				set.insert(id.clone());
			})
			.or_insert_with(|| {
				let mut e = HashSet::new();
				e.insert(id);
				e
			});
		self.sync_local_storage();
	}

	pub fn remove(&mut self, url_in: Url, id: &str) {
		let url = url_in.host_str().expect("Has Host name").to_string();
		self.data.0.entry(url).and_modify(|set| {
			set.remove(id);
		});
		self.sync_local_storage();
	}
}

pub fn get_store(bonus_input: String) -> anyhow::Result<String> {
	let fine = "A".to_string();
	Ok(fine)
}

pub fn setup_store() -> Result<Rc<Store>, JsValue> {
	let store = Store::new("Blocking List")?;
	let rc_store = Rc::new(store);
	Ok(rc_store)
}
