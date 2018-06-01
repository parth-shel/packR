extern crate deflate;

use self::deflate::deflate_bytes;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::metadata;
use std::fs::read_dir;

pub fn pack(file_path: &str) {
	println!("packing: {}", file_path);

	let md = metadata(file_path).unwrap();

	if md.is_file() {
		handle_file(file_path);
	}

	if md.is_dir() {
		handle_dir(file_path);
	}
}

fn handle_file(file_path: &str) {
	// create path to desired file
	let path = Path::new(file_path);
	let display = path.display();

	// open path in read-only mode
	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why.description()),
			Ok(file) => file,
	};

	// read the contents of the file into a string
	let mut data = String::new();
	match file.read_to_string(&mut data) {
		Err(why) => panic!("couldn't read {}: {}", display, why.description()),
			Ok(_) => /*print!("{} contains:\n{}", display, data)*/(),
	}

	// compress the data
	let compressed = deflate_bytes(data.as_bytes());

	// build output file name
	let mut out_file = String::new();
	out_file.push_str(file_path);
	out_file.push_str(".pack");

	// create path to the output file
	let out_path = Path::new(&out_file);
	let out_display = out_path.display();

	// open a file in write-only mode
	let mut file = match File::create(&out_path) {
		Err(why) => panic!("couldn't create {}: {}", out_display, why.description()),
			Ok(file) => file,
	};

	// write the compressed data to the file
	match file.write_all(&compressed) {
		Err(why) => {panic!("couldn't write to {}: {}", out_display, why.description())},
			Ok(_) => println!("successfully wrote to {}", out_display),
	}
}

fn handle_dir(dir_path: &str) {
	let paths = read_dir(dir_path).unwrap();

	for path in paths {
		println!("dir entry: {}", path.unwrap().path().display());	
	}
}
