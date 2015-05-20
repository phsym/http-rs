use std::net::{ToSocketAddrs, TcpStream};
use std::io::{BufReader, BufWriter, Error};

pub mod methods;
pub mod client;
pub mod messages;

pub struct HttpStream {
	sock: TcpStream,
}

impl HttpStream {
	fn new(sock: TcpStream) -> HttpStream {
		return HttpStream {
			sock: sock,
		};
	}
	
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpStream, Error> {
		return Ok(HttpStream::new(try!(TcpStream::connect(addr))));
	}
	
	fn get_reader(&mut self) -> BufReader<&TcpStream> {
		return BufReader::new(&(self.sock));
	}
	
	fn get_writer(&mut self) -> BufWriter<&TcpStream> {
		return BufWriter::new(&(self.sock));
	}
}
