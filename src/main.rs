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
                                  .map_err(|_| "couldn't parse the demo")?;
    println!("{:#?}", demo);

    Ok(())
}
