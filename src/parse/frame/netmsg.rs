use nom::*;

use super::*;

pub const MIN_MESSAGE_LENGTH: i32 = 0;
pub const MAX_MESSAGE_LENGTH: i32 = 65536;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(ref_params<RefParams>,
    do_parse!(
        vieworg:           xyz    >>
        viewangles:        xyz    >>
        forward:           xyz    >>
        right:             xyz    >>
        up:                xyz    >>
        frametime:         le_f32 >>
        time:              le_f32 >>
        intermission:      le_i32 >>
        paused:            le_i32 >>
        spectator:         le_i32 >>
        onground:          le_i32 >>
        waterlevel:        le_i32 >>
        simvel:            xyz    >>
        simorg:            xyz    >>
        viewheight:        xyz    >>
        idealpitch:        le_f32 >>
        cl_viewangles:     xyz    >>
        health:            le_i32 >>
        crosshairangle:    xyz    >>
        viewsize:          le_f32 >>
        punchangle:        xyz    >>
        maxclients:        le_i32 >>
        viewentity:        le_i32 >>
        playernum:         le_i32 >>
        max_entities:      le_i32 >>
        demoplayback:      le_i32 >>
        hardware:          le_i32 >>
        smoothing:         le_i32 >>
        ptr_cmd:           le_i32 >>
        ptr_movevars:      le_i32 >>
        viewport:          i32_4  >>
        next_view:         le_i32 >>
        only_client_draw:  le_i32 >>
        (
            RefParams {
                vieworg,
                viewangles,
                forward,
                right,
                up,
                frametime,
                time,
                intermission,
                paused,
                spectator,
                onground,
                waterlevel,
                simvel,
                simorg,
                viewheight,
                idealpitch,
                cl_viewangles,
                health,
                crosshairangle,
                viewsize,
                punchangle,
                maxclients,
                viewentity,
                playernum,
                max_entities,
                demoplayback,
                hardware,
                smoothing,
                ptr_cmd,
                ptr_movevars,
                viewport,
                next_view,
                only_client_draw,
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(usercmd<UserCmd>,
    do_parse!(
        lerp_msec:       le_i16   >>
        msec:            be_u8    >>
                         take!(1) >>
        viewangles:      xyz      >>
        forwardmove:     le_f32   >>
        sidemove:        le_f32   >>
        upmove:          le_f32   >>
        lightlevel:      be_i8    >>
                         take!(1) >>
        buttons:         le_u16   >>
        impulse:         be_i8    >>
        weaponselect:    be_i8    >>
                         take!(2) >>
        impact_index:    le_i32   >>
        impact_position: xyz      >>
        (
            UserCmd {
                lerp_msec,
                msec,
                viewangles,
                forwardmove,
                sidemove,
                upmove,
                lightlevel,
                buttons,
                impulse,
                weaponselect,
                impact_index,
                impact_position,
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(movevars<MoveVars>,
    do_parse!(
        gravity:           le_f32    >>
        stopspeed:         le_f32    >>
        maxspeed:          le_f32    >>
        spectatormaxspeed: le_f32    >>
        accelerate:        le_f32    >>
        airaccelerate:     le_f32    >>
        wateraccelerate:   le_f32    >>
        friction:          le_f32    >>
        edgefriction:      le_f32    >>
        waterfriction:     le_f32    >>
        entgravity:        le_f32    >>
        bounce:            le_f32    >>
        stepsize:          le_f32    >>
        maxvelocity:       le_f32    >>
        zmax:              le_f32    >>
        wave_height:       le_f32    >>
        footsteps:         le_i32    >>
        sky_name:          take!(32) >>
        rollangle:         le_f32    >>
        rollspeed:         le_f32    >>
        skycolor_r:        le_f32    >>
        skycolor_g:        le_f32    >>
        skycolor_b:        le_f32    >>
        skyvec_x:          le_f32    >>
        skyvec_y:          le_f32    >>
        skyvec_z:          le_f32    >>
        (
            MoveVars {
                gravity,
                stopspeed,
                maxspeed,
                spectatormaxspeed,
                accelerate,
                airaccelerate,
                wateraccelerate,
                friction,
                edgefriction,
                waterfriction,
                entgravity,
                bounce,
                stepsize,
                maxvelocity,
                zmax,
                wave_height,
                footsteps,
                sky_name,
                rollangle,
                rollspeed,
                skycolor_r,
                skycolor_g,
                skycolor_b,
                skyvec_x,
                skyvec_y,
                skyvec_z,
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(net_msg_info<NetMsgInfo>,
    do_parse!(
        timestamp:  le_f32     >>
        ref_params: ref_params >>
        usercmd:    usercmd    >>
        movevars:   movevars   >>
        view:       xyz        >>
        viewmodel:  le_i32     >>
        (
            NetMsgInfo {
                timestamp,
                ref_params,
                usercmd,
                movevars,
                view,
                viewmodel,
            }
        )
    )
);

#[inline]
fn check_msg_length(length: i32) -> IResult<i32, i32, Error> {
    if length < MIN_MESSAGE_LENGTH || length > MAX_MESSAGE_LENGTH {
        IResult::Error(error_code!(ErrorKind::Custom(Error::InvalidNetMsgLength(length))))
    } else {
        IResult::Done(length, length)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub net_msg_data_inner<&[u8], NetMsgData, Error>,
    do_parse!(
        info:                           fix_error!(Error, net_msg_info)                        >>
        incoming_sequence:              fix_error!(Error, le_i32)                              >>
        incoming_acknowledged:          fix_error!(Error, le_i32)                              >>
        incoming_reliable_acknowledged: fix_error!(Error, le_i32)                              >>
        incoming_reliable_sequence:     fix_error!(Error, le_i32)                              >>
        outgoing_sequence:              fix_error!(Error, le_i32)                              >>
        reliable_sequence:              fix_error!(Error, le_i32)                              >>
        last_reliable_sequence:         fix_error!(Error, le_i32)                              >>

        // Can't use length_bytes!() here because it doesn't work with custom error types.
        msg_length:                     flat_map!(fix_error!(Error, le_i32), check_msg_length) >>
        msg:                            fix_error!(Error, take!(msg_length))                   >>
        (
            NetMsgData {
                info,
                incoming_sequence,
                incoming_acknowledged,
                incoming_reliable_acknowledged,
                incoming_reliable_sequence,
                outgoing_sequence,
                reliable_sequence,
                last_reliable_sequence,
                msg,
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub net_msg_data<&[u8], FrameData, Error>,
    map!(net_msg_data_inner, FrameData::NetMsg)
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub net_msg_start_data<&[u8], FrameData, Error>,
    map!(net_msg_data_inner, FrameData::NetMsgStart)
);
