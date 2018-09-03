extern crate uuid;
use uuid::Uuid;

use std::path::Path;
use std::ffi::OsStr;

use std::collections::HashMap;

#[derive(Debug)]
struct Doc {
	filename: String
}

#[derive(Debug)]
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
