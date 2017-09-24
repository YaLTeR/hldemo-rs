#[macro_use]
extern crate error_chain;
extern crate memmap;
extern crate nom;

extern crate hldemo;

use memmap::{Mmap, Protection};
use std::env;

mod errors {
    error_chain! {
        foreign_links {
            ParseError(::hldemo::parse::Error);
        }
    }
}

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let filename = env::args().nth(1).ok_or("no filename")?;
    let mmap = Mmap::open_path(filename, Protection::Read).chain_err(|| "couldn't mmap the file")?;
    let bytes = unsafe { mmap.as_slice() };

    // Parse the demo without frames. This is nearly instant, compared to full parsing which can
    // take a long time.
    let demo = hldemo::parse::demo_without_frames(bytes).to_full_result()
                                                        .map_err(nom_error)
                                                        .chain_err(|| "couldn't parse the demo")?;
    print_demo(&demo);

    Ok(())
}

fn print_demo(demo: &hldemo::Demo) {
    print_header(&demo.header);
    println!("");
    print_directory(&demo.directory);
}

fn print_header(header: &hldemo::Header) {
    println!("Header:");
    println!("\tDemo protocol: {}", header.demo_protocol);
    println!("\tNet protocol: {}", header.net_protocol);
    println!("\tMap name: {}", String::from_utf8_lossy(header.map_name));
    println!("\tGame dir: {}", String::from_utf8_lossy(header.game_dir));
    println!("\tMap CRC: {}", header.map_crc);
    println!("\tDirectory offset: {}", header.directory_offset);
}

fn print_directory(directory: &hldemo::Directory) {
    println!("Directory:");
    for entry in &directory.entries {
        print_entry(entry);
    }
}

fn print_entry(entry: &hldemo::DirectoryEntry) {
    println!("\tEntry:");
    println!("\t\tType: {}", entry.entry_type);
    println!("\t\tDescription: {}", String::from_utf8_lossy(entry.description));
    println!("\t\tFlags: {}", entry.flags);
    println!("\t\tCD track: {}", entry.cd_track);
    println!("\t\tTime: {}", entry.track_time);
    println!("\t\tFrame count: {}", entry.frame_count);
    println!("\t\tOffset: {}", entry.offset);
    println!("\t\tLength: {}", entry.file_length);
}

fn nom_error<I>(err: nom::IError<I, hldemo::parse::Error>) -> Error {
    match err {
        nom::IError::Incomplete(nom::Needed::Size(count)) => {
            format!("need {} more bytes", count).into()
        }
        nom::IError::Incomplete(nom::Needed::Unknown) => format!("need more bytes").into(),
        nom::IError::Error(err) => nom_error_list(&err),
    }
}

fn nom_error_list<I>(err: &nom::Err<I, hldemo::parse::Error>) -> Error {
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
