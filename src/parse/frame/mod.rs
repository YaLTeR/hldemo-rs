//! Demo frame parsing functions.

use nom::*;

use super::*;
use types::*;

pub mod console_command;
pub mod client_data;
pub mod demo_buffer;
pub mod event;
pub mod netmsg;
pub mod sound;
pub mod weapon_anim;

use self::console_command::*;
use self::client_data::*;
use self::demo_buffer::*;
use self::event::*;
use self::netmsg::*;
use self::sound::*;
use self::weapon_anim::*;

/// An enum containing the possible frame types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    DemoStart,
    ConsoleCommand,
    ClientData,
    NextSection,
    Event,
    WeaponAnim,
    Sound,
    DemoBuffer,
    NetMsg,
}

impl From<u8> for FrameType {
    fn from(x: u8) -> Self {
        match x {
            2 => FrameType::DemoStart,
            3 => FrameType::ConsoleCommand,
            4 => FrameType::ClientData,
            5 => FrameType::NextSection,
            6 => FrameType::Event,
            7 => FrameType::WeaponAnim,
            8 => FrameType::Sound,
            9 => FrameType::DemoBuffer,
            _ => FrameType::NetMsg,
        }
    }
}

/// A demo frame header.
///
/// Every frame starts with a header, followed by frame data depending on the frame type.
pub struct FrameHeader {
    pub frame_type: FrameType,
    pub time: f32,
    pub frame: i32,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub frame_header<FrameHeader>,
    do_parse!(
        frame_type: map!(be_u8, From::from) >>
        time:       le_f32                  >>
        frame:      le_i32                  >>
        (
            FrameHeader {
                frame_type,
                time,
                frame
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub frame_next_section<Frame>,
    map_res!(frame_header, |FrameHeader { frame_type, time, frame }| {
        if frame_type == FrameType::NextSection {
            Ok(Frame {
                   time,
                   frame,
                   data: FrameData::NextSection,
               })
        } else {
            Err(())
        }
    })
);

#[inline]
pub fn frame_data(input: &[u8], frame_type: FrameType) -> IResult<&[u8], FrameData, Error> {
    match frame_type {
        FrameType::DemoStart => IResult::Done(input, FrameData::DemoStart),
        FrameType::ConsoleCommand => fix_error!(input, Error, console_command_data),
        FrameType::ClientData => fix_error!(input, Error, client_data_data),
        FrameType::NextSection => IResult::Done(input, FrameData::NextSection),
        FrameType::Event => fix_error!(input, Error, event_data),
        FrameType::WeaponAnim => fix_error!(input, Error, weapon_anim_data),
        FrameType::Sound => fix_error!(input, Error, sound_data),
        FrameType::DemoBuffer => fix_error!(input, Error, demo_buffer_data),
        FrameType::NetMsg => net_msg_data(input),
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub frame<&[u8], Frame, Error>,
    do_parse!(
        frame_header: fix_error!(Error, frame_header)            >>
        data:         call!(frame_data, frame_header.frame_type) >>
        (
            Frame {
                time: frame_header.time,
                frame: frame_header.frame,
                data
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub frames<&[u8], Vec<Frame>, Error>,
    add_parse_error!(Frames,
        map!(many_till!(frame, frame_next_section),
             |(mut fs, f)| {
                 fs.push(f);
                 fs
             })
    )
);

#[inline]
pub fn offset_frames(input: &[u8], offset: usize) -> IResult<&[u8], Vec<Frame>, Error> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(input,
                fix_error!(Error, take!(offset)) >>
        frames: frames                           >>
        (frames)
    )
}
