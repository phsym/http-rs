#[macro_use] extern crate http;

use std::io::prelude::*;
use std::str;

use http::methods::Method;
use http::client::*;

fn main() {
	let mut http = HttpClient::new("www.google.com:80").unwrap();
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
}
