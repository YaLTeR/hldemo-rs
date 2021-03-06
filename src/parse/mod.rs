//! Nom parsing functions for demo parsing.

use nom::*;

#[macro_use]
mod macros;
mod demo;
pub mod directory;
pub mod frame;
pub mod header;

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
            display("invalid demo protocol: {} (only protocol {} is supported)",
                    protocol,
                    header::SUPPORTED_DEMO_PROTOCOL)
        }
        InvalidDirectoryEntryCount(count: i32) {
            display("invalid directory entry count: {} (expected from {} to {})",
                    count,
                    directory::MIN_ENTRY_COUNT,
                    directory::MAX_ENTRY_COUNT)
        }
        InvalidFrameType(frame_type: u8) {
            display("invalid frame type: {} (expected from 0 to {})",
                    frame_type,
                    frame::MAX_FRAME_TYPE)
        }
        InvalidNetMsgLength(length: i32) {
            display("invalid netmsg length: {} (expected from {} to {})",
                    length,
                    frame::netmsg::MIN_MESSAGE_LENGTH,
                    frame::netmsg::MAX_MESSAGE_LENGTH)
        }
    }
}

// Required by nom.
impl From<u32> for Error {
    fn from(_: u32) -> Self {
        // We aren't using this.
        unreachable!()
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
