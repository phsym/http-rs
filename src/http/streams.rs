//! Http I/O streams definitions

use std::net::{ToSocketAddrs, TcpStream};
use std::io::{BufReader, BufWriter, Error, Read, Write};

pub trait Open {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<Self, Error>;
}

pub trait Stream<S: Read+Write> {
	fn new_reader(&mut self) -> BufReader<S>;
	fn new_writer(&mut self) -> BufWriter<S>;
}

/// HttpStream for unsecured HTTP Input/Output
pub type HttpStream = TcpStream;

impl Open for HttpStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpStream, Error> {
		return Ok(try!(TcpStream::connect(addr)));
	}
}

impl Stream<HttpStream> for HttpStream {
	fn new_reader(&mut self) -> BufReader<HttpStream> {
		return BufReader::new(self.try_clone().unwrap());
	}
	
	fn new_writer(&mut self) -> BufWriter<HttpStream> {
		return BufWriter::new(self.try_clone().unwrap());
	}
}

use openssl::ssl::{SslContext, SslStream, SslMethod};

/// HttpsStream for HTTPS Input/Output
pub type HttpsStream = SslStream<TcpStream>;

impl Open for HttpsStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpsStream, Error> {
		let ctx = SslContext::new(SslMethod::Tlsv1).unwrap();
		let sock = try!(TcpStream::connect(addr));
		let stream = SslStream::new(&ctx, sock).unwrap();
		return Ok(stream);
	}
}

impl Stream<HttpsStream> for HttpsStream {
	fn new_reader(&mut self) -> BufReader<HttpsStream> {
		return BufReader::new(self.try_clone().unwrap());
	}
	
	fn new_writer(&mut self) -> BufWriter<HttpsStream> {
		return BufWriter::new(self.try_clone().unwrap());
	}
}
