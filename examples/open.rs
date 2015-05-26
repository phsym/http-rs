#[macro_use] extern crate http;

use http::{Protocol, open};

fn main() {
	let mut _http = open(Protocol::HTTP, "www.google.com:80").unwrap();
	// Do something with _http
	
	let mut _https = open(Protocol::HTTPS, "www.google.com:443").unwrap();
	// Do something with _https
}