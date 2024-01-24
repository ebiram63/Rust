use std::collections::BTreeMap;


let mut map = BTreeMap::new();
map.insert("alice", 100);
assert_eq!(map.get(&"alice"), Some(&100));
assert_eq!(map.get(&"bob"), None);


let maybe_value = map.get(&"alice");
match maybe_value {
	Some(value) => {
		// do something with the `value`
	},
	None => {
		// perhaps return an error since there was no value there
	}
}

pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new()
		}
	}
}


