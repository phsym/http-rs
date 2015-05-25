//! Http I/O streams definitions

use std::net::{ToSocketAddrs, TcpStream};
use std::io::{Error, Read, Write};

/// Represent a type that can be opened (ie connected) to a remote `SocketAddress`
pub trait Open {
	/// Create a new Instance of `Self` connected to `addr`
	fn open<A: ToSocketAddrs>(addr: A) -> Result<Self, Error>;
}

/// A trait representing an openable read/write stream
pub trait Stream: Read+Write+Open {}

/// HttpStream for unsecured HTTP Input/Output
pub type HttpStream = TcpStream;
impl Stream for HttpStream{}

impl Open for HttpStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpStream, Error> {
		return Ok(try!(TcpStream::connect(addr)));
	}
}

use openssl::ssl::{SslContext, SslStream, SslMethod};

/// HttpsStream for secured HTTPS Input/Output
pub type HttpsStream = SslStream<TcpStream>;
impl Stream for HttpsStream{}

impl Open for HttpsStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpsStream, Error> {
		let ctx = SslContext::new(SslMethod::Tlsv1).unwrap();
		let sock = try!(TcpStream::connect(addr));
		let stream = SslStream::new(&ctx, sock).unwrap();
		return Ok(stream);
	}
}
