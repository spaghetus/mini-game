use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// Gets a state struct from the state hashmap. Returns None if the key is absent or invalid
pub fn get_state<T: DeserializeOwned>(state: &HashMap<String, Value>, key: &str) -> Option<T> {
	match state.get(&key.to_string()) {
		Some(v) => match serde_json::from_value(v.clone()) {
			Ok(v) => Some(v),
			Err(_) => {
				eprintln!(
					"Warning: Attempted to access key {} which was of an unexpected type {}.",
					key, v
				);
				None
			}
		},
		None => None,
	}
}

/// Puts a state struct into the state hashmap.
pub fn put_state<T: Serialize>(state: &mut HashMap<String, Value>, key: &str, value: T) {
	state.insert(
		key.to_string(),
		serde_json::to_value(value).expect("Failed to serialize"),
	);
}
