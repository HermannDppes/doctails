use clap::{Arg, App, SubCommand, ArgMatches};
use std::path::Path;

pub fn doctails<'a, 'b>() -> App<'a, 'b> {
	let app = App::new("doctails")
		.about("Pins Tails on Docs.")
		.author("Hermann Döppes <hermannd.ouml.ppes@gmail.com>")
		.version(crate_version!());
	app.subcommand(adopt())
}

fn adopt<'a, 'b>() -> App<'a, 'b> {
	let sc = SubCommand::with_name("adopt")
		.about("Adopt a new Doc.");
	let path = Arg::with_name("PATH")
		.help("Path to the document / blob that should be taken in")
		.required(true);
	sc.arg(path)
}

pub fn run<'a>(pack: &'a mut super::Pack<'a>, matches: &'a ArgMatches<'a>) {
	if let Some(sub_matches) = matches.subcommand_matches("adopt") {
		pack.adopt(Path::new(sub_matches.value_of("PATH").unwrap()));
	}
}
