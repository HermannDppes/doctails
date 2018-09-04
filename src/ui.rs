use std::path::Path;
use std::fs::File;
use super::*;
use std::error::Error;

pub enum Action<'a> {
	Adopt(&'a Path)
}

pub struct Config<'a> {
	pub action: Action<'a>
}

const PACKPATH: &str = "storage/pack";

pub fn put_pack(pack: &Pack) -> Result<(), Box<Error>> {
	let mut packfile = File::create(PACKPATH)?;
	serde_cbor::to_writer(&mut packfile, pack)?;
	Ok(())
}

pub fn get_pack() -> Result<Pack, Box<Error>> {
	let pack = match File::open(PACKPATH) {
		Ok(file) => serde_cbor::from_reader(file)?,
		// FIXME: Only create new pack if file not found,
		//        abort otherwise
		Err(_) => Pack::new()
	};
	Ok(pack)
}
