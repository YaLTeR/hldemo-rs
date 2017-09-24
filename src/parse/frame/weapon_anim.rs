use nom::*;

use super::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub weapon_anim_data<FrameData>,
    do_parse!(
        anim: le_i32 >>
        body: le_i32 >>
        (
            FrameData::WeaponAnim(
                WeaponAnimData {
                    anim,
                    body,
                }
            )
        )
    )
);
