#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate memmap;
#[macro_use]
extern crate nom;

use memmap::{Mmap, Protection};
use std::env;

use errors::*;

mod errors {
    error_chain!{}
}

mod parser;
mod types;

quick_main!(run);

fn run() -> Result<()> {
    let filename = env::args().nth(1).ok_or("no filename")?;
    let mmap = Mmap::open_path(filename, Protection::Read).chain_err(|| "couldn't mmap the file")?;
    let bytes = unsafe { mmap.as_slice() };

    let demo = parser::demo(bytes).to_full_result()
        .map_err(|err| {
            match err {
                nom::IError::Error(e) => format!("couldn't parse the demo: {}", nom_error_string(&e)),
                nom::IError::Incomplete(nom::Needed::Size(s)) => format!("couldn't parse the demo: need {} more bytes", s),
                nom::IError::Incomplete(nom::Needed::Unknown) => format!("couldn't parse the demo: need more bytes")
            }
        })?;
    println!("{:#?}", demo);

    Ok(())
}

fn nom_error_string(err: &nom::ErrorKind) -> &'static str {
    match nom::error_to_u32(err) {
        0 => "the magic value didn't match",
        _ => "unknown error"
    }
}
