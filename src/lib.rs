//! Low level HTTP toolkit library
//!
//! Feature "ssl" enable secured connection support with openssl.
//! This feature is enabled by default but can be disabled
#[cfg(feature="ssl")]
extern crate openssl;
mod http;
pub use http::*;
