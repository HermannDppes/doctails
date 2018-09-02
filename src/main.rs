extern crate uuid;
use uuid::Uuid;

#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
	let app = App::new("doctails")
		.about("Pins Tails on Docs.")
		.author("Hermann Döppes <hermannd.ouml.ppes@gmail.com>")
		.version(crate_version!());
	let adopt = SubCommand::with_name("adopt")
		.about("Adopt a new Doc.")
		.arg(Arg::with_name("PATH")
			.help("Path to the document / blob that should be taken in")
			.required(true)
		);
	let matches = app
		.subcommand(adopt)
		.get_matches();
	let uuid = Uuid::new_v4();
	println!("{}", uuid.hyphenated());
}
