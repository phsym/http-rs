use std::net::{SocketAddr, ToSocketAddrs, TcpStream};
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::{Iter, Keys};
use std::io::{Error, BufWriter, ErrorKind};

use super::*;
use super::methods::Method;
use super::messages::HttpReply;
use super::constants::properties;

/// A simple and low-level HTTP client implementation
pub struct HttpClient {
	addr: SocketAddr,
	header: HashMap<String, String>,
	stream: Option<HttpStream>
}

impl HttpClient {
	
	/// Create a new HTTP client that will send requests to `addr`.
	/// # Example
	/// ```no_run
	/// use http::client::HttpClient;
	/// let mut client = HttpClient::new("www.google.com:80");
	/// // Send some requests
	/// ```
	pub fn new<A: ToSocketAddrs>(addr: A) -> Result<HttpClient, Error> {
		let address = option!(try!(addr.to_socket_addrs()).next(), "Cannot resolve address");
		let client = HttpClient{
			addr: address,
			header: HashMap::new(),
			stream: None
		};
		return Ok(client);
	}
	
	/// Get a property from client permanent header
	pub fn get_property(&self, key: &String) -> Option<&String> {
		return self.header.get(key);
	}
	
	/// Set a property in permanent header
	pub fn set_property(&mut self, key: String, value: String) {
		self.header.insert(key, value);
	}
	
	/// Remove a property from permanent header
	pub fn unset_property(&mut self, key: &String) {
		self.header.remove(key);
	}
	
	/// Return an iterator over properties names from permanent header
	pub fn get_properties_name(&self) -> Keys<String, String> {
		return self.header.keys();
	}
	
	/// Return an iterator over properties from permanent header
	pub fn iter(&self) -> Iter<String, String> {
		return self.header.iter();
	}
	
	/// Open an `HttpStream` to remote host
	fn connect(&mut self) -> Result<&mut HttpStream, Error> {
		self.stream = Some(try!(HttpStream::open(self.addr)));
		return Ok(self.stream.as_mut().unwrap());
	}
	
	fn update_properties(&self, header: Option<&HashMap<String, String>>) -> HashMap<String, String> {
		let mut hdr = match header {
			Some(h) => h.clone(),
			None => HashMap::new()
		};
		for (k, v) in &self.header {
			if ! hdr.contains_key(k) {
				hdr.insert(k.clone(), v.clone());
			}
		}
		return hdr;
	}
	
	pub fn send_stream(&mut self, method: Method, path: &str, header: Option<&HashMap<String, String>>) -> Result<BufWriter<&TcpStream>, Error> {
		let hdr = self.update_properties(header);
		let mut stream = try!(self.connect());
		let mut w = stream.get_writer();
		{
			let mut writer = &mut w;
			try!(writer.write(method.as_bytes()));
			try!(writer.write(b" "));
			try!(writer.write(path.as_bytes()));
			try!(writer.write(b"\r\n"));
			
			//Write header
			for (k, v) in hdr {
				try!(writer.write(k.as_bytes()));
				try!(writer.write(b": "));
				try!(writer.write(v.as_bytes()));
				try!(writer.write(b"\r\n"));
			}
			try!(writer.write(b"\r\n"));
		}
		return Ok(w);
	}
	
	pub fn get_reply(&mut self) -> Result<HttpReply, Error> {
		let stream = match self.stream.as_mut() {
			Some(s) => s,
			None => return Err(Error::new(ErrorKind::NotConnected, "Cannot get reply since no stream is opened"))
		};
		return HttpReply::parse(stream.get_reader());
	}
	
	pub fn send(&mut self, method: Method, path: &str, header: Option<&HashMap<String, String>>, data: Option<&[u8]>) -> Result<HttpReply, Error> {
		{
			let mut hdr = match header {
				Some(h) => h.clone(),
				None => HashMap::new()
			};
			if let Some(d) = data {
				hdr.insert(properties::CONTENT_LENGTH.to_string(), d.len().to_string());
			}
			let mut writer = try!(self.send_stream(method, path, Some(&hdr)));
			if let Some(d) = data {
				try!(writer.write(d));
			}
			try!(writer.flush());
		}
		return self.get_reply();
	}
}