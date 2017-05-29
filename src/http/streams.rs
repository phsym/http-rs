//! Http I/O streams definitions

use std::net::{ToSocketAddrs, TcpStream};
use std::io::{Error, Read, Write};
#[cfg(feature="ssl")]
use std::io::ErrorKind;

/// Represent a type that can be opened (ie connected) to a remote `SocketAddress`
pub trait Open: Sized {
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

#[cfg(feature="ssl")]
use openssl::ssl::{SslStream, SslMethod, SslConnectorBuilder, SslVerifyMode};

/// HttpsStream for secured HTTPS Input/Output. Only available if "ssl" feature is enabled
#[cfg(feature="ssl")]
pub type HttpsStream = SslStream<TcpStream>;
#[cfg(feature="ssl")]
impl Stream for HttpsStream{}

#[cfg(feature="ssl")]
impl Open for HttpsStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpsStream, Error> {
		let sock = try!(TcpStream::connect(addr));
		let mut builder = match SslConnectorBuilder::new(SslMethod::tls()) {
			Ok(s) => s,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot create SSL connector : {}", e)))
		};
		//TODO: Do not skip verifications
		builder.builder_mut().set_verify(SslVerifyMode::empty());
		let stream = match builder.build().danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication(sock) {
			Ok(s) => s,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot create SSL stream : {}", e)))
		};
		return Ok(stream);
	}
}
