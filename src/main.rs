#[macro_use]
extern crate clap;

extern crate doctails;
use doctails::Pack;

mod cli;

mod ui {
	use std;

	pub enum Action<'a> {
		Adopt(&'a std::path::Path)
	}

	pub struct Config<'a> {
		pub pack: (),
		pub action: Action<'a>
	}
}

fn main() {
	let matches = cli::doctails().get_matches();
	let config = cli::config(&matches);
	cli::run(config);
}
