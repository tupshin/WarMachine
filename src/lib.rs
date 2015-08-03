#![feature(custom_derive, plugin)]
#![plugin(clippy)]
//#![plugin(serde_macros)]
#![plugin(serde)]
#[macro_use] 
extern crate rustful;

extern crate cassandra;

pub use server::{cql,http};


pub mod server {
	pub mod http;
	pub mod cql;
}

pub mod client {
	pub mod cassandra;
}

pub mod conf {
	pub mod toml;
	pub mod docopt;
}

#[test]
fn it_works() {
}
