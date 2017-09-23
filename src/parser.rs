use nom::*;
use std::str;

use errors::*;
use types::*;

named_args!(take_c_str(count: usize)<&str>,
    map_res!(flat_map!(take!(count), take_until!(&[0][..])), str::from_utf8)
);

named!(magic<()>, do_parse!(tag!("HLDEMO") >> take!(2) >> ()));

named!(
    header<Header>,
    do_parse!(
        call!(magic)                             >>
        demo_protocol:    le_i32                 >>
        net_protocol:     le_i32                 >>
        map_name:         call!(take_c_str, 260) >>
        game_dir:         call!(take_c_str, 260) >>
        map_crc:          le_i32                 >>
        directory_offset: le_i32                 >>
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

fn check_count(count: i32) -> Result<i32> {
    const MIN_DIR_ENTRY_COUNT: i32 = 1;
    const MAX_DIR_ENTRY_COUNT: i32 = 1024;

    if count < MIN_DIR_ENTRY_COUNT || count > MAX_DIR_ENTRY_COUNT {
        Err("invalid directory entry count".into())
    } else {
        Ok(count)
    }
}

named!(
    directory<Directory>,
    do_parse!(
        entries: length_count!(map_res!(le_i32, check_count), call!(entry)) >>
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
            }
        )
    )
);

named_args!(offset_directory(offset: usize)<Directory>,
    do_parse!(
        take!(offset)               >>
        directory: call!(directory) >>
        (directory)
    )
);

named!(pub demo<Demo>,
    do_parse!(
        header:    peek!(call!(header))                                      >>
        directory: call!(offset_directory, header.directory_offset as usize) >>
        (
            Demo {
                header,
                directory,
            }
        )
    )
);
