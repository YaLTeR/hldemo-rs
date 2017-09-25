//! A parser for Goldsource demo files (this includes Half-Life and its mods) written in Rust using
//! [nom](https://crates.io/crates/nom).
//!
//! Supports parsing demos completely as well as just header and directory when the frame data
//! isn't needed. Exports both raw nom parsing functions and wrappers which hide all nom-related
//! types and provide convenient errors.
//!
//! # Examples
//! Check the `examples` folder for more complete program examples which output various data from
//! demos.
//!
//! ```no_run
//! # use std::error::Error;
//! #
//! # fn try_main() -> Result<(), Box<Error>> {
//! extern crate hldemo;
//!
//! use std::fs::File;
//! use std::io::Read;
//!
//! let mut bytes = Vec::new();
//! let mut f = File::open("demo.dem")?;
//! f.read_to_end(&mut bytes);
//!
//! let demo = hldemo::Demo::parse(&bytes)?;
//! #
//! #     Ok(())
//! # }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/hldemo/0.1.0")]
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quick_error;

pub mod errors;
pub mod parse;
mod types;

pub use types::*;
