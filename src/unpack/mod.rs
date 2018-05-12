extern crate inflate;

use self::inflate::inflate_bytes;
// use std::str::from_utf8;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn unpack(file_path: &str) {
	println!("unpacking: {}", file_path);
	
	// create path to desired file
	let path = Path::new(file_path);
	let display = path.display();

	// open path in read-only mode
	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why.description()),
			Ok(file) => file,
	};

	// read the contents of the file
	let mut data = Vec::new();
	match file.read_to_end(&mut data) {
		Err(why) => panic!("couldn't read {}: {}", display, why.description()),
			Ok(_) => (),
	}

	// build output file name
	let mut out_file = String::new();
	out_file.push_str(file_path);
	out_file.push_str(".unpack");

	handle_file(&data, &out_file);
}

fn handle_file(contents: &[u8], out_file: &str) {
	// decompress the data
	let encoded = contents;
	let decoded = inflate_bytes(&encoded).unwrap();

	// create path to the output file
	let out_path = Path::new(&out_file);
	let out_display = out_path.display();

	// open a file in write-only mode
	let mut file = match File::create(&out_path) {
		Err(why) => panic!("couldn't create {}: {}", out_display, why.description()),
			Ok(file) => file,
	};

	// write the decompressed data to the file
	match file.write_all(&decoded) {
		Err(why) => {panic!("couldn't write to {}: {}", out_display, why.description())},
			Ok(_) => println!("successfully wrote to {}", out_display),
	}
}
