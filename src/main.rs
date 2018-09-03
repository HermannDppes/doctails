#[macro_use]
extern crate clap;

extern crate doctails;
use doctails::Pack;

mod cli;

fn main() {
	let matches = cli::doctails().get_matches();

	let mut pack = Pack::new();

	cli::run(&mut pack, &matches);
}
