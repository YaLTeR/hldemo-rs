use nom::*;

use super::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(event_args<EventArgs>,
    do_parse!(
        flags:        le_i32 >>
        entity_index: le_i32 >>
        origin:       xyz    >>
        angles:       xyz    >>
        velocity:     xyz    >>
        ducking:      le_i32 >>
        fparam1:      le_f32 >>
        fparam2:      le_f32 >>
        iparam1:      le_i32 >>
        iparam2:      le_i32 >>
        bparam1:      le_i32 >>
        bparam2:      le_i32 >>
        (
            EventArgs {
                  flags,
                  entity_index,
                  origin,
                  angles,
                  velocity,
                  ducking,
                  fparam1,
                  fparam2,
                  iparam1,
                  iparam2,
                  bparam1,
                  bparam2,
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub event_data<FrameData>,
    do_parse!(
        flags: le_i32     >>
        index: le_i32     >>
        delay: le_f32     >>
        args:  event_args >>
        (
            FrameData::Event(
                EventData {
                    flags,
                    index,
                    delay,
                    args,
                }
            )
        )
    )
);
