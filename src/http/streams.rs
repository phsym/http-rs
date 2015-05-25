//! Http I/O streams definitions

use std::net::{ToSocketAddrs, TcpStream};
use std::io::{BufReader, BufWriter, Error, Read, Write};
use std::ops::Deref;

/// Represent a type that can be opened (ie connected) to a remote `SocketAddress`
pub trait Open {
	/// Create a new Instance of `Self` connected to `addr`
	fn open<A: ToSocketAddrs>(addr: A) -> Result<Self, Error>;
}

pub trait Stream: Read+Write+Sized {
	/// Build a new `BufReader` to self
	fn new_reader(&mut self) -> BufReader<Self> {
		return BufReader::new(self.copy());
	}
	/// Build a new `BufWriter` to self
	fn new_writer(&mut self) -> BufWriter<Self> {
		return BufWriter::new(self.copy());
	}
	/// Create a copy of the stream
	fn copy(&self) -> Self;
}

/// HttpStream for unsecured HTTP Input/Output
pub type HttpStream = TcpStream;

impl Open for HttpStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpStream, Error> {
		return Ok(try!(TcpStream::connect(addr)));
	}
}

impl Stream for HttpStream {
	fn copy(&self) -> HttpStream {
		return self.try_clone().unwrap();
	}
}

use openssl::ssl::{SslContext, SslStream, SslMethod};

/// HttpsStream for secured HTTPS Input/Output
pub type HttpsStream = SslStream<TcpStream>;

impl Open for HttpsStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpsStream, Error> {
		let ctx = SslContext::new(SslMethod::Tlsv1).unwrap();
		let sock = try!(TcpStream::connect(addr));
		let stream = SslStream::new(&ctx, sock).unwrap();
		return Ok(stream);
	}
}

impl Stream for HttpsStream {
	fn copy(&self) -> HttpsStream {
		return self.try_clone().unwrap();
	}
}


impl <S: Open> Open for Box<S> {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<Box<S>, Error> {
		return Ok(Box::new(try!(S::open(addr))));
	}
}

impl <S: Stream> Stream for Box<S> {
	fn copy(&self) -> Box<S> {
		return Box::new(self.deref().copy());
	}
}