use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use web_sys::window;

pub fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
    if let Some(storage) = get_storage() {
        let sto_res = storage.get_item(key);
        if let Ok(sto_opt) = sto_res {
            if let Some(sto) = sto_opt {
                let obj_res = serde_json::from_str::<T>(sto.as_str());
                if let Ok(obj) = obj_res {
					return Some(obj);
                }
            }
        }
    }
    None
}

pub fn save<T: Serialize>(key: &str, time: T) {
	if let Some(storage) = get_storage() {
		if let Ok(json) = serde_json::to_string(&time) {
			let _res = storage.set_item(key, &json);
		}
	}
}

fn get_storage() -> Option<web_sys::Storage> {
	if let Some(window) = window() {
		if let Ok(storage_opt) = window.local_storage() {
			return storage_opt;
		}
	}
	None
}