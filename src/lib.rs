#![doc(html_root_url = "https://docs.rs/hldemo/0.1.0")]
#![recursion_limit = "1024"]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate quick_error;

pub mod parse;
mod types;

pub use types::*;
