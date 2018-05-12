extern crate deflate;
extern crate inflate;

use self::deflate::deflate_bytes;
use self::inflate::inflate_bytes;
use std::str::from_utf8;

pub fn pack(file_path: &str) {
	println!("packing: {}", file_path);

	let data = "Hello World!";
	let compressed = deflate_bytes(data.as_bytes());
	println!("{:?}", compressed);

	let encoded = compressed;
	let decoded = inflate_bytes(&encoded).unwrap();
	println!("{}", from_utf8(&decoded).unwrap()); // prints "Hello, world"
}
