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

    let demo = hldemo::parse::demo(bytes).to_full_result()
                                         .map_err(nom_error)
                                         .chain_err(|| "couldn't parse the demo")?;
    print_frames(&demo);

    Ok(())
}

fn print_frames(demo: &hldemo::Demo) {
    for (i, entry) in demo.directory.entries.iter().enumerate() {
        println!("Entry {}:", i);

        for frame in &entry.frames {
            print_frame(frame);
        }

        println!("");
    }
}

fn print_frame(frame: &hldemo::Frame) {
    println!("\tf={} t={} type={}{}",
             frame.frame,
             frame.time,
             frame_type_string(&frame.data),
             frame_extra_info(&frame.data));
}

fn frame_type_string(data: &hldemo::FrameData) -> &'static str {
    match *data {
        hldemo::FrameData::DemoStart => "DemoStart",
        hldemo::FrameData::ConsoleCommand(_) => "ConsoleCommand",
        hldemo::FrameData::ClientData(_) => "ClientData",
        hldemo::FrameData::NextSection => "NextSection",
        hldemo::FrameData::Event(_) => "Event",
        hldemo::FrameData::WeaponAnim(_) => "WeaponAnim",
        hldemo::FrameData::Sound(_) => "Sound",
        hldemo::FrameData::DemoBuffer(_) => "DemoBuffer",
        hldemo::FrameData::NetMsg(_) => "NetMsg",
    }
}

fn frame_extra_info(data: &hldemo::FrameData) -> String {
    match *data {
        hldemo::FrameData::ConsoleCommand(ref d) => {
            format!(" command=`{}`",
                    String::from_utf8_lossy(d.command.split(|&x| x == 0).next().unwrap()))
        }
        hldemo::FrameData::DemoBuffer(ref d) => format!(" size={}", d.buffer.len()),
        hldemo::FrameData::NetMsg(ref d) => format!(" size={}", d.msg.len()),
        _ => "".to_string(),
    }
}

fn nom_error<I>(err: nom::IError<I, hldemo::parse::Error>) -> Error {
    match err {
        nom::IError::Incomplete(nom::Needed::Size(count)) => {
            format!("need {} more bytes", count).into()
        }
        nom::IError::Incomplete(nom::Needed::Unknown) => "need more bytes".into(),
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
