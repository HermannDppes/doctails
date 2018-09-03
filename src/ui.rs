use std::path::Path;

pub enum Action<'a> {
	Adopt(&'a Path)
}

pub struct Config<'a> {
	pub pack: (),
	pub action: Action<'a>
}
