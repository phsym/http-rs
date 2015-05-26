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
	/// Secured HTTP
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
/// // Do something with http
/// # drop(http);
/// let mut https = open(Protocol::HTTPS, "www.google.com:443").unwrap();
/// // Do something with https;
/// # drop(https);
/// ```
pub fn open<'a, A: ToSocketAddrs>(protocol: Protocol, addr: A) -> Result<Box<Http>, Error> {
	let cli: Box<Http> = match protocol {
		Protocol::HTTP => Box::new(try!(HttpClient::new(addr))),
		Protocol::HTTPS => Box::new(try!(HttpsClient::new(addr)))
	};
	return Ok(cli);
}
