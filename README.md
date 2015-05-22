![License](http://img.shields.io/badge/license-BSD-lightgrey.svg)
[![Build Status](https://travis-ci.org/phsym/http-rs.svg)](https://travis-ci.org/phsym/http-rs)

# http-rs

*Copyright &copy; 2015 Pierre-Henri Symoneaux*

> THIS SOFTWARE IS DISTRIBUTED WITHOUT ANY WARRANTY <br>
> Check LICENSE.txt file for more information. <br>

A simple and low level http client toolkit written in Rust.
This is just a pet project I use to discover and learn Rust. If you need a more complete and well supported HTTP library,
I advise you to have a look at [Teepee](http://teepee.rs/) or at [Hyper](https://github.com/hyperium/hyper)

# How to build
As usual with Cargo project, simply run

> cargo build

And to build html documentation, run

> cargo doc

# How to use
More often, you will include the library as a dependency to your project. In order to do this, add the following lines to your **Cargo.toml** file :

```toml
[dependencies.http-rs]
git = "https://github.com/phsym/http-rs.git"

```

Then you can start using it the following way :

```rust
#[macro_use] extern crate http;
use http::client::HttpClient;
use http::methods::Method;

fn main() {
	let mut http = HttpClient::new("www.google.fr:80").unwrap();
	match http.send(Method::GET, /, None, None) {
		Ok(reply) => {},// Do something with the reply
		Err(e) => panic!("Cannot send request : {}", e)
	}
}
```

Additional examples are provided in documentation.
