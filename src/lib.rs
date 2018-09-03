extern crate uuid;
use uuid::Uuid;

use std::path::Path;

use std::collections::HashMap;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(Debug, Serialize, Deserialize)]
struct Doc {
	filename: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
	docs: HashMap<Uuid, Doc>
}

impl Pack {
	pub fn adopt(&mut self, path: &Path) {
		let uuid = Uuid::new_v4();
		let filename = path.file_name().unwrap()
			.to_string_lossy()
			.into_owned();
		let doc = Doc {filename};
		self.docs.insert(uuid, doc);
		println!("We are now {:?}", self);
	}

	pub fn new() -> Pack {
		Pack {
			docs: HashMap::new()
		}
	}
}
