use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Error, ErrorKind, BufRead};
use std::str::FromStr;

use super::methods::Method;

pub struct HttpRequest<'r> {
	method: Method,
	path: &'r str,
	header: HashMap<&'r str, &'r str>,
	data: [u8]
}

pub struct HttpReply {
	pub code: u32,
	pub header: HashMap<String, String>,
	reader: BufReader<TcpStream>
}

impl HttpReply {
	fn new(code: u32, header: HashMap<String, String>, reader: BufReader<TcpStream>) -> HttpReply {
		return HttpReply{code: code, header: header, reader: reader};
	}
	
	pub fn parse(mut reader: BufReader<TcpStream>) -> Result<HttpReply, Error> {
		let mut code: u32;
		let mut header = HashMap::new();
		{
			let mut line = String::new();
			try!(reader.read_line(&mut line));
			let mut splt = line.split(" ");
			splt.next();
			code = match u32::from_str(splt.next().unwrap()) {
				Ok(n) => n,
				Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot parse HTTP code : {}", e)))
			};
			
			//Parse header
			let mut line = String::new();
			loop {
				try!(reader.read_line(&mut line));
				line = line.trim().to_string();
				if line.is_empty() {break;}
					{
						let mut splt = line.split(": ");
						header.insert(splt.next().unwrap().to_string(), splt.next().unwrap().to_string());
					}
				line.clear();
			}
		}
		
		let reply = HttpReply::new(code, header, reader);
		return Ok(reply);
	}
	
	pub fn get_reader(&mut self) -> &mut BufReader<TcpStream> {
		return &mut self.reader;
	}
}