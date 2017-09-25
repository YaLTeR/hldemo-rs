//! Functions for parsing demos.

use nom::*;

#[macro_use]
mod macros;
mod demo;
mod directory;
mod frame;
mod header;

pub use self::demo::{demo, demo_without_frames};

quick_error! {
    /// This type represents possible errors that can occur when parsing demos.
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum Error {
        Header {
            display("couldn't parse the demo header")
        }
        Directory {
            display("couldn't parse the demo directory")
        }
        Frames {
            display("couldn't parse the demo frames")
        }
        InvalidMagic {
            display("invalid magic value")
        }
        InvalidDemoProtocol(protocol: i32) {
            display("invalid demo protocol: {} (only protocol 5 is supported)", protocol)
        }
        InvalidDirectoryEntryCount(count: i32) {
            display("invalid directory entry count: {} (expected from 1 to 1024)", count)
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(xyz<[f32; 3]>,
    do_parse!(
        x: le_f32 >>
        y: le_f32 >>
        z: le_f32 >>
        ([x, y, z])
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(i32_4<[i32; 4]>,
    do_parse!(
        a: le_i32 >>
        b: le_i32 >>
        c: le_i32 >>
        d: le_i32 >>
        ([a, b, c, d])
    )
);
