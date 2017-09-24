use nom::*;

use super::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub sound_data<FrameData>,
    do_parse!(
        channel:     le_i32                >>
        sample:      length_bytes!(le_i32) >>
        attenuation: le_f32                >>
        volume:      le_f32                >>
        flags:       le_i32                >>
        pitch:       le_i32                >>
        (
            FrameData::Sound(
                SoundData {
                    channel,
                    sample,
                    attenuation,
                    volume,
                    flags,
                    pitch,
                }
            )
        )
    )
);
