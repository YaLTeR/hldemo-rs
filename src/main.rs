#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate memmap;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quick_error;

use memmap::{Mmap, Protection};
use std::env;

use errors::*;

mod errors {
    error_chain! {
        foreign_links {
            ParseError(::parse::Error);
        }
    }
}

mod parse;
mod types;

quick_main!(run);

fn run() -> Result<()> {
    let filename = env::args().nth(1).ok_or("no filename")?;
    let mmap = Mmap::open_path(filename, Protection::Read).chain_err(|| "couldn't mmap the file")?;
    let bytes = unsafe { mmap.as_slice() };

    let demo = parse::demo(bytes).to_full_result()
                                 .map_err(nom_error)
                                 .chain_err(|| "couldn't parse the demo")?;
    // println!("{:#?}", demo);
    println!("Parsed.");

    Ok(())
}

fn nom_error<I>(err: nom::IError<I, parse::Error>) -> Error {
    match err {
        nom::IError::Incomplete(nom::Needed::Size(count)) => {
            format!("need {} more bytes", count).into()
        }
        nom::IError::Incomplete(nom::Needed::Unknown) => format!("need more bytes").into(),
        nom::IError::Error(err) => nom_error_list(&err),
    }
}

fn nom_error_list<I>(err: &nom::Err<I, parse::Error>) -> Error {
    let v = nom::error_to_list(err);
    let mut iter = v.into_iter()
                    .rev()
                    .filter_map(|x| if let nom::ErrorKind::Custom(inner) = x {
                                    Some(inner)
                                } else {
                                    None
                                });
    let mut err = Error::from(iter.next().unwrap());

    for e in iter {
        err = Error::with_chain(err, Error::from(e));
    }

    err
}
