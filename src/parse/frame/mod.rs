use nom::*;

use super::*;
use types::*;

mod console_command;
mod client_data;
mod demo_buffer;
mod event;
mod netmsg;
mod sound;
mod weapon_anim;

use self::console_command::*;
use self::client_data::*;
use self::demo_buffer::*;
use self::event::*;
use self::netmsg::*;
use self::sound::*;
use self::weapon_anim::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameType {
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

struct FrameHeader {
    frame_type: FrameType,
    time: f32,
    frame: i32,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(frame_header<FrameHeader>,
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
named!(frame_next_section<Frame>,
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
fn frame_data(input: &[u8], frame_type: FrameType) -> IResult<&[u8], FrameData> {
    match frame_type {
        FrameType::DemoStart => IResult::Done(input, FrameData::DemoStart),
        FrameType::ConsoleCommand => console_command_data(input),
        FrameType::ClientData => client_data_data(input),
        FrameType::NextSection => IResult::Done(input, FrameData::NextSection),
        FrameType::Event => event_data(input),
        FrameType::WeaponAnim => weapon_anim_data(input),
        FrameType::Sound => sound_data(input),
        FrameType::DemoBuffer => demo_buffer_data(input),
        FrameType::NetMsg => net_msg_data(input),
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(frame<Frame>,
    do_parse!(
        frame_header: frame_header                               >>
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
named!(frames<&[u8], Vec<Frame>, Error>,
    add_parse_error!(Frames,
        fix_error!(Error,
                   map!(many_till!(frame, frame_next_section),
                        |(mut fs, f)| {
                            fs.push(f);
                            fs
                        })
        )
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
