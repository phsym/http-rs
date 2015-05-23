//! Defines some usefull constants

/// Properties names constant for HTTP headers
pub mod properties {
	pub const CONTENT_LENGTH: &'static str = "Content-Length";
	pub const CONTENT_TYPE: &'static str = "Content-Type";
	pub const ACCEPT: &'static str = "Accept";
	pub const DATE: &'static str = "Date";
	pub const LOCATION: &'static str = "Location";
}

/// Mime types constants
pub mod mimetypes {
	pub const TEXT_PLAIN: &'static str = "text/plain";
	pub const APP_JSON: &'static str = "application/json";
	pub const APP_XML: &'static str = "application/xml";
	pub const APP_OCTET_STREAM: &'static str = "application/octet-stream";
}
