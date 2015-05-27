#[macro_use] extern crate http;

use std::io::prelude::*;
use std::str;

use http::methods::Method;
use http::client::*;

#[cfg(feature="ssl")]
fn main() {
	let mut http = HttpsClient::new("www.google.com:443").unwrap();
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

#[cfg(not(feature="ssl"))]
fn main() {
	panic!("ssl feature not enabled");
}
