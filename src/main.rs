extern crate uuid;
use uuid::Uuid;

use std::path::Path;

#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

fn adopt(path: &Path) {
	let uuid = Uuid::new_v4();
	println!("{} adopted as {}", path.to_str().unwrap(), uuid.hyphenated());
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

	if let Some(sub_matches) = matches.subcommand_matches("adopt") {
		adopt(Path::new(sub_matches.value_of("PATH").unwrap()));
	}
}
