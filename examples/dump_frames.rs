#[macro_use]
extern crate error_chain;
extern crate hldemo;
extern crate memmap;

use memmap::{Mmap, Protection};
use std::env;

mod errors {
    error_chain!{}
}
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let filename = env::args().nth(1).ok_or("no filename")?;
    let mmap = Mmap::open_path(filename, Protection::Read).chain_err(|| "couldn't mmap the file")?;
    let bytes = unsafe { mmap.as_slice() };

    let demo = hldemo::Demo::parse(bytes).chain_err(|| "couldn't parse the demo")?;
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
