#[macro_use] extern crate http;

use http::{Protocol, open};

#[cfg(feature="ssl")]
fn use_https() {
	let mut _https = open(Protocol::HTTPS, "www.google.com:443").unwrap();
	// Do something with _https
}

#[cfg(not(feature="ssl"))]
fn use_https() {
	panic!("ssl feture not enabled");
}

fn main() {
	let mut _http = open(Protocol::HTTP, "www.google.com:80").unwrap();
	// Do something with _http
	use_https();
}