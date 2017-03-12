#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod api;
mod errors;
mod krate;

pub use krate::Crate;
