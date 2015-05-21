pub enum Method {
	GET,
	POST,
	PUT,
	DELETE
}

impl Method {
	pub fn as_slice(&self) -> &'static str {
		return match *self {
			Method::GET => "GET",
			Method::POST => "POST",
			Method::PUT => "PUT",
			Method::DELETE => "DELETE"
		}
	}
	
	pub fn as_bytes(&self) -> &[u8] {
		return self.as_slice().as_bytes();
	}
}