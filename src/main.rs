use std::env;

#[macro_use] extern crate lazy_static;

mod pack;
mod unpack;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 3 {
		println!("usage: pack_r <mode> <file_path>");
		panic!("expected args!");
	}

	let flags =  &args[1];
	let file_path = &args[2];

	match flags.as_ref() {
		"pack" => pack::pack(file_path),
		"unpack" => unpack::unpack(file_path),
		_ => panic!("invalid args!"),
	}
}
