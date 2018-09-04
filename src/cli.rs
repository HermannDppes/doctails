use clap::{Arg, App, SubCommand, ArgMatches};
use std::path::Path;
use super::*;

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

pub fn config<'a>(
	matches: &'a ArgMatches<'a>
) -> ui::Config<'a> {
	let action = get_action(matches);
	ui::Config {action}
}

fn get_action<'a>(matches: &'a ArgMatches<'a>) -> ui::Action<'a> {
	match matches.subcommand() {
	("adopt", Some(sub_matches)) => {
		let path = Path::new(sub_matches.value_of("PATH").unwrap());
		ui::Action::Adopt(path)
	},
	(&_, _) =>
		panic!("No known action specified")
	}
}

pub fn run(config: ui::Config) {
	let mut pack = ui::get_pack().expect("Failed to read pack from disk");
	match config.action {
		ui::Action::Adopt(path) => pack.adopt(path)
	}
	ui::put_pack(&pack).expect("Failed to write pack to disk");
}
