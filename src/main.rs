extern crate uuid;
use uuid::Uuid;

use std::path::Path;

use std::ffi::OsStr;

use std::collections::HashMap;

#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

#[derive(Debug)]
struct Doc<'a> {
	filename: &'a OsStr
}

#[derive(Debug)]
struct Pack<'a> {
	docs: HashMap<Uuid, Doc<'a>>
}

impl<'a> Pack<'a> {
	fn adopt(&'a mut self, path: &'a Path) {
		let uuid = Uuid::new_v4();
		let doc = Doc {
			filename: path.file_name().unwrap()
		};
		self.docs.insert(uuid, doc);
		println!("We are now {:?}", self);
	}
}

fn main() {
	let app = App::new("doctails")
		.about("Pins Tails on Docs.")
		.author("Hermann Döppes <hermannd.ouml.ppes@gmail.com>")
		.version(crate_version!());
	let sc_adopt = SubCommand::with_name("adopt")
		.about("Adopt a new Doc.")
		.arg(Arg::with_name("PATH")
			.help("Path to the document / blob that should be taken in")
			.required(true)
		);
	let matches = app
		.subcommand(sc_adopt)
		.get_matches();

	let mut pack = Pack {
		docs: HashMap::new()
	};

	if let Some(sub_matches) = matches.subcommand_matches("adopt") {
		pack.adopt(Path::new(sub_matches.value_of("PATH").unwrap()));
	}
}
