#[macro_use] mod macros;
pub mod methods;
pub mod constants;
pub mod client;
pub mod messages;
pub mod streams;

use std::net::ToSocketAddrs;
use std::io::Error;
use self::client::*;

/// Protocol implementations
pub enum Protocol {
	/// Unsecured HTTP
	HTTP,
	/// Secured HTTP. Only available with "ssl" feature
	#[cfg(feature="ssl")]
	HTTPS,
}

/// Create a new client. The implementation which may differ according to the `protocol` is hidden
/// behind a boxed `Http` trait
///
/// # Example :
/// ```no_run
/// use http::{open, Protocol};
///
/// let mut http = open(Protocol::HTTP, "www.google.com:80").unwrap();
/// // If ssl feature is enabled, Protocol::HTTPS van be used to open a secured connection if "ssl" feature is enabled
/// // Do something with http
/// # drop(http);
/// ```
pub fn open<A: ToSocketAddrs>(protocol: Protocol, addr: A) -> Result<Box<Http>, Error> {
	let cli: Box<Http> = match protocol {
		Protocol::HTTP => Box::new(try!(HttpClient::new(addr))),
		#[cfg(feature="ssl")]
		Protocol::HTTPS => Box::new(try!(HttpsClient::new(addr)))
	};
	return Ok(cli);
}
