use std::io::prelude::*;
use std::collections::HashMap;
use std::str;

mod http;
use http::methods::Method;
use http::client::HttpClient;

fn main() {
	
	let mut http = HttpClient::new("www.google.fr:80");
//	let mut http = HttpClient::new("127.0.0.1:80");
	let mut hdr = HashMap::new();
	hdr.insert("test", "toto");
	
	match http.send(Method::GET, "/", Some(&hdr), Some(b"tatayoyo")) {
		Ok(ref mut res) => {
			println!("code : {}", res.code);
			
			println!("\nHeader : ");
			for (k, v) in &res.header {
				println!("{} => {}", k, v);
			}
			
			let mut data = [0u8; 1024];
			if let Err(e) = res.get_reader().read(&mut data){
				panic!("Cannot read data {}", e);
			}
			let mystr = str::from_utf8(&data).unwrap();
			
			println!("\nContent : \n{}", mystr);
		},
		Err(ref e) => panic!("Cannot send request : {}", e)
	}
	
	http.send(Method::GET, "/", Some(&hdr), Some(b"tatayoyo"));
}
