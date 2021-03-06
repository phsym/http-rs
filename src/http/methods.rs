//! Methods definitions

/// Supported HTTP methods
pub enum Method {
	GET,
	POST,
	PUT,
	DELETE,
	HEAD,
	TRACE,
	OPTIONS,
	PATCH,
	CONNECT
}

impl Method {
	/// Return a static string representation of the method name
	pub fn as_slice(&self) -> &'static str {
		return match *self {
			Method::GET => "GET",
			Method::POST => "POST",
			Method::PUT => "PUT",
			Method::DELETE => "DELETE",
			Method::HEAD => "HEAD",
			Method::TRACE => "TRACE",
			Method::OPTIONS => "OPTIONS",
			Method::PATCH => "PATCH",
			Method::CONNECT => "CONNECT"
		}
	}
	
	/// Return the method name representation as an utf8 encoded byte slice
	pub fn as_bytes(&self) -> &[u8] {
		return self.as_slice().as_bytes();
	}
}