#[macro_use]
extern crate error_chain;
extern crate hldemo;
extern crate memmap;

use memmap::MmapOptions;
use std::env;
use std::fs::File;

mod errors {
    error_chain!{}
}
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let filename = env::args().nth(1).ok_or("no filename")?;
    let file = File::open(filename).chain_err(|| "couldn't open the file")?;
    let mmap = unsafe {
        MmapOptions::new().map(&file)
                          .chain_err(|| "couldn't mmap the file")?
    };

    // Parse the demo without frames. This is nearly instant, compared to full parsing which can
    // take a long time.
    let demo = hldemo::Demo::parse_without_frames(&mmap).chain_err(|| "couldn't parse the demo")?;
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
    println!("\tMap name: {}",
             String::from_utf8_lossy(header.map_name.split(|&x| x == 0).next().unwrap()));
    println!("\tGame dir: {}",
             String::from_utf8_lossy(header.game_dir.split(|&x| x == 0).next().unwrap()));
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
    println!("\t\tDescription: {}",
             String::from_utf8_lossy(entry.description.split(|&x| x == 0).next().unwrap()));
    println!("\t\tFlags: {}", entry.flags);
    println!("\t\tCD track: {}", entry.cd_track);
    println!("\t\tTime: {}", entry.track_time);
    println!("\t\tFrame count: {}", entry.frame_count);
    println!("\t\tOffset: {}", entry.offset);
    println!("\t\tLength: {}", entry.file_length);
}
