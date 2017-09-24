use nom::*;

use super::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub client_data_data<FrameData>,
    do_parse!(
        origin:      xyz    >>
        viewangles:  xyz    >>
        weapon_bits: le_i32 >>
        fov:         le_f32 >>
        (
            FrameData::ClientData(
                ClientDataData {
                    origin,
                    viewangles,
                    weapon_bits,
                    fov,
                }
            )
        )
    )
);
