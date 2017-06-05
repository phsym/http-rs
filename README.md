![License](http://img.shields.io/badge/license-BSD-lightgrey.svg)
[![Build Status](https://travis-ci.org/phsym/http-rs.svg)](https://travis-ci.org/phsym/http-rs)

# http-rs

[Documentation](http://phsym.github.io/http-rs/doc)

*Copyright &copy; 2015 Pierre-Henri Symoneaux*

> THIS SOFTWARE IS DISTRIBUTED WITHOUT ANY WARRANTY <br>
> Check LICENSE.txt file for more information. <br>

A simple and low level http/https client toolkit written in Rust.
This is just a pet project I used to discover and learn Rust. I advise you to have a look at [Hyper](https://github.com/hyperium/hyper) instead.
Please consider that **no design is stable for now here**. I'm just a rust noob who still have a lot to learn on this beautifull new language.

# How to build

The library has an optional "ssl" feature that enable HTTPS over SSL. SSL support is enabled by default but can be disabled if not needed.
SSL support depends on [rust-openssl](https://github.com/sfackler/rust-openssl) which requires both the OpenSSL runtime libraries and headers to be built.

## Without SSL support
As usual with Cargo project, simply run

> cargo build --no-default-features

And to build html documentation, run

> cargo doc --no-default-features

## With SSL support
Building with SSL support require beeing able to build [rust-openssl](https://github.com/sfackler/rust-openssl).
Also, gcc will be required.
Please have a look on [rust-openssl](https://github.com/sfackler/rust-openssl) and [gcc-rs](https://github.com/alexcrichton/gcc-rs) README files for detailed information about building opennssl.
If environment configuration satisfies everything needed to build SSL support, then to build simply run

> cargo build

And to build html documentation, run

> cargo doc

# How to use
More often, you will include the library as a dependency to your project. In order to do this, add the following lines to your **Cargo.toml** file :

```toml
[dependencies.http-rs]
git = "https://github.com/phsym/http-rs.git"
# To disable SSL support, default features need to be disabled by uncommenting the following line
# default-features = false

```

Then you can start using it the following way :

```rust
#[macro_use] extern crate http;
use http::client::*;
use http::methods::Method;

fn main() {
	let mut http = HttpClient::new("www.google.com:80").unwrap();
	match http.send(Method::GET, /, None, None) {
		Ok(reply) => {},// Do something with the reply
		Err(e) => panic!("Cannot send request : {}", e)
	}
	// The same is possible on HTTPS :
	let mut https = HttpsClient::new("www.google.com:443").unwrap();
	// (...)
}
```

If you want to create an HTTP client hiding which implementation is used (either HttpClient or HttpsClient), you
can get a boxed `Http` trait with the `http::open` function like this :

```rust
#[macro_use] extern crate http;
use http::{Protocol, open};

fn main {
	let mut http = open(Protocol::HTTP, "www.google.com:80").unwrap();
	// Do something with http
	let mut https = open(Protocol::HTTPS, "www.google.com:443").unwrap();
	// Do something with https;
}
```

Additional examples are provided in documentation and in [examples](./examples/) directory
