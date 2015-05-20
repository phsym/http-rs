use std::net::{SocketAddr, ToSocketAddrs};
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::Error;

use super::*;
use super::methods::Method;
use super::messages::HttpReply;

pub struct HttpClient {
	addr: SocketAddr,
	header: HashMap<String, String>,
	stream: Option<HttpStream>
}

impl HttpClient {
	pub fn new<A: ToSocketAddrs>(addr: A) -> HttpClient {
		let client = HttpClient{
			addr: addr.to_socket_addrs().unwrap().next().unwrap(),
			header: HashMap::new(),
			stream: None
		};
		return client;
	}
	
	fn connect(&mut self) -> Result<&mut HttpStream, Error> {
		self.stream = Some(try!(HttpStream::open(self.addr)));
		return Ok(self.stream.as_mut().unwrap());
	}
	
	pub fn send(&mut self, method: Method, path: &str, header: Option<&HashMap<&str, &str>>, data: Option<&[u8]>) -> Result<HttpReply, Error> {
		let mut stream = try!(self.connect());
		{
			let mut writer = &mut stream.get_writer();
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
		return HttpReply::parse(stream.get_reader());
	}
}