use std::net::{SocketAddr, ToSocketAddrs, TcpStream};
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::{Error, BufWriter, ErrorKind};

use super::*;
use super::methods::Method;
use super::messages::HttpReply;

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
	
	/// Open a `TcpStream` to remote host
	fn connect(&mut self) -> Result<&mut HttpStream, Error> {
		self.stream = Some(try!(HttpStream::open(self.addr)));
		return Ok(self.stream.as_mut().unwrap());
	}
	
	pub fn send_stream(&mut self, method: Method, path: &str, header: Option<&HashMap<String, String>>) -> Result<BufWriter<&TcpStream>, Error> {
		let mut stream = try!(self.connect());
		let mut w = stream.get_writer();
		{
			let mut writer = &mut w;
			try!(writer.write(method.as_bytes()));
			try!(writer.write(b" "));
			try!(writer.write(path.as_bytes()));
			try!(writer.write(b"\r\n"));
			
			//Write header
			//TODO : Merge with client header
			if let Some(hdr) = header {
				for (k, v) in hdr {
					try!(writer.write(k.as_bytes()));
					try!(writer.write(b": "));
					try!(writer.write(v.as_bytes()));
					try!(writer.write(b"\r\n"));
				}
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
			let mut hdr;
			if let Some(h) = header { hdr = h.clone(); }
			else { hdr = HashMap::new(); }
			if let Some(d) = data {
				hdr.insert("Content-Length".to_string(), d.len().to_string());
			}
			let mut writer = try!(self.send_stream(method, path, header));
			if let Some(d) = data {
				try!(writer.write(d));
			}
			try!(writer.flush());
		}
		return self.get_reply();
	}
	
	pub fn send_string(&mut self, method: Method, path: &str, header: Option<&HashMap<String, String>>, string: &str) -> Result<HttpReply, Error> {
		return self.send(method, path, header, Some(string.as_bytes()));
	}
}