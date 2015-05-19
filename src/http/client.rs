use std::net::{SocketAddr, ToSocketAddrs};
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::Error;

use super::*;
use super::methods::Method;
use super::messages::{HttpReply, HttpRequest};

pub struct HttpClient<'r> {
	addr: SocketAddr,
	header: HashMap<&'r str, &'r str>,
}

impl <'i> HttpClient<'i> {
	pub fn new<'r, A: ToSocketAddrs>(addr: A) -> HttpClient<'r> {
		let client = HttpClient{
			addr: addr.to_socket_addrs().unwrap().next().unwrap(),
			header: HashMap::new()
		};
		return client;
	}
	
	pub fn send(&self, method: Method, path: &str, header: Option<&HashMap<&str, &str>>, data: Option<&[u8]>) -> Result<HttpReply, Error> {
		let mut stream = try!(HttpStream::open(self.addr));
		{
			let mut writer = &mut stream.to_writer();
			try!(writer.write(method.as_bytes()));
			try!(writer.write(b" "));
			try!(writer.write(path.as_bytes()));
			try!(writer.write(b"\r\n"));
			
			//TODO: Write header and length
			if header.is_some() {
				for (k, v) in header.unwrap() {
					try!(writer.write(k.as_bytes()));
					try!(writer.write(b": "));
					try!(writer.write(v.as_bytes()));
					try!(writer.write(b"\r\n"));
				}
			}
			
			try!(writer.write(b"\r\n"));
			if data.is_some() {
				try!(writer.write(data.unwrap()));
			}
			try!(writer.flush());
		}
		return HttpReply::parse(stream.to_reader());
	}
}