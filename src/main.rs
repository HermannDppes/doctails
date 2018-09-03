extern crate doctails;
use doctails::Pack;

#[macro_use]
extern crate clap;

extern crate serde;
extern crate serde_cbor;

mod cli;
mod ui;

fn main() {
	let matches = cli::doctails().get_matches();
	let config = cli::config(&matches);
	cli::run(config);
}
