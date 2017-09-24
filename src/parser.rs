use nom;
use nom::*;
use std::str;

use errors::ParseError;
use types::*;

named_args!(take_c_str(count: usize)<&str>,
    map_res!(flat_map!(take!(count), take_until!(&[0][..])), str::from_utf8)
);

named!(xyz<[f32; 3]>,
       do_parse!(x: le_f32 >> y: le_f32 >> z: le_f32 >> ([x, y, z])));

named!(i32_4<[i32; 4]>,
       do_parse!(a: le_i32 >> b: le_i32 >> c: le_i32 >> d: le_i32 >> ([a, b, c, d])));

named!(magic<&[u8], (), ParseError>,
    add_return_error!(
        nom::ErrorKind::Custom(ParseError::InvalidMagic),
        fix_error!(ParseError, do_parse!(tag!("HLDEMO") >> take!(2) >> ()))
    )
);

#[inline]
fn check_demo_protocol(protocol: i32) -> IResult<i32, i32, ParseError> {
    if protocol == 5 {
        IResult::Done(protocol, protocol)
    } else {
        IResult::Error(error_code!(nom::ErrorKind::Custom(ParseError::InvalidDemoProtocol)))
    }
}

named!(demo_protocol<&[u8], i32, ParseError>,
    flat_map!(fix_error!(ParseError, le_i32), check_demo_protocol)
);

named!(
    header<&[u8], Header, ParseError>,
    do_parse!(
                          magic                                          >>
        demo_protocol:    demo_protocol                                  >>
        net_protocol:     fix_error!(ParseError, le_i32)                 >>
        map_name:         fix_error!(ParseError, call!(take_c_str, 260)) >>
        game_dir:         fix_error!(ParseError, call!(take_c_str, 260)) >>
        map_crc:          fix_error!(ParseError, le_i32)                 >>
        directory_offset: fix_error!(ParseError, le_i32)                 >>
        (
            Header {
                demo_protocol,
                net_protocol,
                map_name,
                game_dir,
                map_crc,
                directory_offset,
            }
        )
    )
);

#[inline]
fn check_count(count: i32) -> Result<i32, ()> {
    const MIN_DIR_ENTRY_COUNT: i32 = 1;
    const MAX_DIR_ENTRY_COUNT: i32 = 1024;

    if count < MIN_DIR_ENTRY_COUNT || count > MAX_DIR_ENTRY_COUNT {
        Err(())
    } else {
        Ok(count)
    }
}

named!(
    directory<Directory>,
    do_parse!(
        entries: length_count!(map_res!(le_i32, check_count), entry) >>
        (
            Directory {
                entries
            }
        )
    )
);

named!(
    entry<DirectoryEntry>,
    do_parse!(
        entry_type:  le_i32                >>
        description: call!(take_c_str, 64) >>
        flags:       le_i32                >>
        cd_track:    le_i32                >>
        track_time:  le_f32                >>
        frame_count: le_i32                >>
        offset:      le_i32                >>
        file_length: le_i32                >>
        (
            DirectoryEntry {
                entry_type,
                description,
                flags,
                cd_track,
                track_time,
                frame_count,
                offset,
                file_length,
                frames: Vec::new(),
            }
        )
    )
);

#[inline]
fn offset_directory(input: &[u8], offset: usize) -> IResult<&[u8], Directory, ParseError> {
    fix_error!(input,
               ParseError,
               do_parse!(take!(offset) >> directory: directory >> (directory)))
}

fn offset_directory_with_frames(input: &[u8],
                                offset: usize)
                                -> IResult<&[u8], Directory, ParseError> {
    match offset_directory(input, offset) {
        IResult::Done(_, mut directory) => {
            for entry in &mut directory.entries {
                entry.frames = match add_return_error!(input,
                                                       nom::ErrorKind::Custom(ParseError::Frames),
                                                       call!(offset_frames, entry.offset as usize))
                {
                    IResult::Done(_, frames) => frames,
                    IResult::Incomplete(needed) => return IResult::Incomplete(needed),
                    IResult::Error(error) => return IResult::Error(error),
                };
            }

            IResult::Done(input, directory)
        }
        other => other,
    }
}

named!(frames<Vec<Frame>>,
       map!(many_till!(frame_not_next_section, frame_next_section),
            |(mut fs, f)| {
                fs.push(f);
                fs
            }));

#[inline]
fn offset_frames(input: &[u8], offset: usize) -> IResult<&[u8], Vec<Frame>, ParseError> {
    fix_error!(input,
               ParseError,
               do_parse!(take!(offset) >> frames: frames >> (frames)))
}

named!(frame_header<(u8, f32, i32)>,
       do_parse!(frame_type: be_u8 >> time: le_f32 >> frame: le_i32 >> (frame_type, time, frame)));

named!(
    frame_next_section<Frame>,
    map_res!(frame_header, |(frame_type, time, frame)| {
        if frame_type == 5 {
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

named!(
    frame_not_next_section<Frame>,
    do_parse!(
        frame_header: frame_header >>
        data: call!(frame_data_not_next_section, frame_header.0) >>
        (
            Frame {
                time: frame_header.1,
                frame: frame_header.2,
                data
            }
        )
    )
);

fn frame_data_not_next_section(input: &[u8], frame_type: u8) -> IResult<&[u8], FrameData> {
    match frame_type {
        2 => IResult::Done(input, FrameData::DemoStart),
        3 => console_command_data(input),
        4 => client_data_data(input),
        5 => IResult::Error(error_code!(nom::ErrorKind::Custom(0))),
        6 => event_data(input),
        7 => weapon_anim_data(input),
        8 => sound_data(input),
        9 => demo_buffer_data(input),
        _ => net_msg_data(input),
    }
}

named!(
    console_command_data<FrameData>,
    map!(call!(take_c_str, 64), |command| {
        FrameData::ConsoleCommand(ConsoleCommandData { command })
    })
);

named!(client_data_data<FrameData>,
       do_parse!(origin: xyz >> viewangles: xyz >> weapon_bits: le_i32 >> fov: le_f32
                     >> (FrameData::ClientData(ClientDataData {
                                                   origin,
                                                   viewangles,
                                                   weapon_bits,
                                                   fov,
                                               }))));

named!(event_args<EventArgs>,
       do_parse!(flags: le_i32 >> entity_index: le_i32 >> origin: xyz >> angles: xyz
                     >> velocity: xyz >> ducking: le_i32 >> fparam1: le_f32
                     >> fparam2: le_f32 >> iparam1: le_i32 >> iparam2: le_i32
                     >> bparam1: le_i32 >> bparam2: le_i32 >> (EventArgs {
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
                 })));

named!(event_data<FrameData>,
       do_parse!(flags: le_i32 >> index: le_i32 >> delay: le_f32 >> args: event_args
                     >> (FrameData::Event(EventData {
                                              flags,
                                              index,
                                              delay,
                                              args,
                                          }))));

named!(weapon_anim_data<FrameData>,
       do_parse!(anim: le_i32 >> body: le_i32 >> (FrameData::WeaponAnim(WeaponAnimData {
                                                                            anim,
                                                                            body,
                                                                        }))));

named!(
    sound_data<FrameData>,
    do_parse!(
        channel: le_i32 >>
        sample: length_bytes!(le_i32) >>
        attenuation: le_f32 >>
        volume: le_f32 >>
        flags: le_i32 >>
        pitch: le_i32 >>
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

named!(
    demo_buffer_data<FrameData>,
    map!(length_bytes!(le_i32), |buffer| {
        FrameData::DemoBuffer(DemoBufferData { buffer })
    })
);

#[inline]
fn check_msg_length(length: i32) -> Result<i32, ()> {
    const MIN_MESSAGE_LENGTH: i32 = 0;
    const MAX_MESSAGE_LENGTH: i32 = 65536;

    if length < MIN_MESSAGE_LENGTH || length > MAX_MESSAGE_LENGTH {
        Err(())
    } else {
        Ok(length)
    }
}

named!(
    net_msg_data<FrameData>,
    do_parse!(
        info: net_msg_info >>
        incoming_sequence: le_i32 >>
        incoming_acknowledged: le_i32 >>
        incoming_reliable_acknowledged: le_i32 >>
        incoming_reliable_sequence: le_i32 >>
        outgoing_sequence: le_i32 >>
        reliable_sequence: le_i32 >>
        last_reliable_sequence: le_i32 >>
        msg: length_bytes!(map_res!(le_i32, check_msg_length)) >>
        (
            FrameData::NetMsg(
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
    )
);

named!(net_msg_info<NetMsgInfo>,
       do_parse!(timestamp: le_f32 >> ref_params: ref_params >> usercmd: usercmd
                     >> movevars: movevars >> view: xyz >> viewmodel: le_i32
                     >> (NetMsgInfo {
                         timestamp,
                         ref_params,
                         usercmd,
                         movevars,
                         view,
                         viewmodel,
                     })));

named!(ref_params<RefParams>,
       do_parse!(vieworg: xyz >> viewangles: xyz >> forward: xyz >> right: xyz >> up: xyz
                     >> frametime: le_f32 >> time: le_f32 >> intermission: le_i32
                     >> paused: le_i32 >> spectator: le_i32 >> onground: le_i32
                     >> waterlevel: le_i32 >> simvel: xyz >> simorg: xyz
                     >> viewheight: xyz >> idealpitch: le_f32 >> cl_viewangles: xyz
                     >> health: le_i32 >> crosshairangle: xyz >> viewsize: le_f32
                     >> punchangle: xyz >> maxclients: le_i32 >> viewentity: le_i32
                     >> playernum: le_i32 >> max_entities: le_i32
                     >> demoplayback: le_i32 >> hardware: le_i32
                     >> smoothing: le_i32 >> ptr_cmd: le_i32
                     >> ptr_movevars: le_i32 >> viewport: i32_4
                     >> next_view: le_i32 >> only_client_draw: le_i32
                     >> (RefParams {
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
                     })));

named!(usercmd<UserCmd>,
       do_parse!(lerp_msec: le_i16 >> msec: be_u8 >> take!(1) >> viewangles: xyz
                     >> forwardmove: le_f32 >> sidemove: le_f32 >> upmove: le_f32
                     >> lightlevel: be_i8 >> take!(1) >> buttons: le_u16
                     >> impulse: be_i8 >> weaponselect: be_i8 >> take!(2)
                     >> impact_index: le_i32 >> impact_position: xyz
                     >> (UserCmd {
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
                     })));

named!(
    movevars<MoveVars>,
    do_parse!(
        gravity: le_f32 >>
        stopspeed: le_f32 >>
        maxspeed: le_f32 >>
        spectatormaxspeed: le_f32 >>
        accelerate: le_f32 >>
        airaccelerate: le_f32 >>
        wateraccelerate: le_f32 >>
        friction: le_f32 >>
        edgefriction: le_f32 >>
        waterfriction: le_f32 >>
        entgravity: le_f32 >>
        bounce: le_f32 >>
        stepsize: le_f32 >>
        maxvelocity: le_f32 >>
        zmax: le_f32 >>
        wave_height: le_f32 >>
        footsteps: le_i32 >>
        sky_name: call!(take_c_str, 32) >>
        rollangle: le_f32 >>
        rollspeed: le_f32 >>
        skycolor_r: le_f32 >>
        skycolor_g: le_f32 >>
        skycolor_b: le_f32 >>
        skyvec_x: le_f32 >>
        skyvec_y: le_f32 >>
        skyvec_z: le_f32 >>
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

named!(pub demo_without_frames<&[u8], Demo, ParseError>,
    do_parse!(
        header: add_return_error!(nom::ErrorKind::Custom(ParseError::Header), peek!(header)) >>
        directory: add_return_error!(nom::ErrorKind::Custom(ParseError::Directory),
                                     call!(offset_directory, header.directory_offset as usize)) >>
        (
            Demo {
                header,
                directory,
            }
        )
    )
);

named!(pub demo<&[u8], Demo, ParseError>,
    do_parse!(
        header: add_return_error!(nom::ErrorKind::Custom(ParseError::Header), peek!(header)) >>
        directory: add_return_error!(
            nom::ErrorKind::Custom(ParseError::Directory),
            call!(offset_directory_with_frames, header.directory_offset as usize)
        ) >>
        (
            Demo {
                header,
                directory,
            }
        )
    )
);
