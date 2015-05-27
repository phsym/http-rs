#[macro_use] extern crate http;

use std::io::prelude::*;
use std::str;

use http::methods::Method;
use http::client::*;
//use http::{open, Protocol};

#[allow(dead_code)]
fn main() {
	let mut http = HttpsClient::new("www.google.com:80").unwrap();
//	let mut http = HttpClient::new("127.0.0.1:80").unwrap();
	http.set_property(str!("perm"), str!("test"));
	let hdr = smap!(
		"test" => "toto",
		"foo" => "bar",
		"num" => 12
		);
	debug!("MAP : {:?}", hdr);
	
	match http.send(Method::GET, "/", Some(&hdr), Some(b"tatayoyo")) {
		Ok(ref mut res) => {
			debug!("Reply : \n{:?}", res);
			let mut data = [0u8; 1024];
			if let Err(e) = res.get_reader().read(&mut data){
				panic!("Cannot read data {}", e);
			}
			let mystr = str::from_utf8(&data).unwrap();
			
			println!("Content : \n{}", mystr);
		},
		Err(ref e) => panic!("Cannot send request : {}", e)
	}
	
	let mut repl = http.send(Method::GET, "/", Some(&hdr), Some(b"tatayoyo")).unwrap();
	let my_str = repl.read_string().unwrap();
	println!("New data : {}", my_str);
}
