extern crate uuid;
use uuid::Uuid;

use std::path::Path;

use std::ffi::OsStr;

use std::collections::HashMap;

#[derive(Debug)]
struct Doc<'a> {
	filename: &'a OsStr
}

#[derive(Debug)]
pub struct Pack<'a> {
	docs: HashMap<Uuid, Doc<'a>>
}

impl<'a> Pack<'a> {
	pub fn adopt(&'a mut self, path: &'a Path) {
		let uuid = Uuid::new_v4();
		let doc = Doc {
			filename: path.file_name().unwrap()
		};
		self.docs.insert(uuid, doc);
		println!("We are now {:?}", self);
	}

	pub fn new() -> Pack<'a> {
		Pack {
			docs: HashMap::new()
		}
	}
}
