//! Demo directory parsing functions.

use nom::*;

use super::*;
use super::frame::*;
use types::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub entry<DirectoryEntry>,
    do_parse!(
        entry_type:  le_i32    >>
        description: take!(64) >>
        flags:       le_i32    >>
        cd_track:    le_i32    >>
        track_time:  le_f32    >>
        frame_count: le_i32    >>
        offset:      le_i32    >>
        file_length: le_i32    >>
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
fn check_count(count: i32) -> IResult<usize, usize, Error> {
    const MIN_DIR_ENTRY_COUNT: i32 = 1;
    const MAX_DIR_ENTRY_COUNT: i32 = 1024;

    if count < MIN_DIR_ENTRY_COUNT || count > MAX_DIR_ENTRY_COUNT {
        IResult::Error(error_code!(ErrorKind::Custom(Error::InvalidDirectoryEntryCount(count))))
    } else {
        IResult::Done(count as usize, count as usize)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub directory<&[u8], Directory, Error>,
    add_parse_error!(Directory,
        do_parse!(
            // Can't use length_count!() here because it doesn't work with custom error types.
            length:  flat_map!(fix_error!(Error, le_i32), check_count) >>
            entries: fix_error!(Error, count!(entry, length as usize)) >>
            (
                Directory {
                    entries
                }
            )
        )
    )
);

#[inline]
pub fn offset_directory(input: &[u8], offset: usize) -> IResult<&[u8], Directory, Error> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(input,
                   fix_error!(Error, take!(offset)) >>
        directory: directory                        >>
        (directory)
    )
}

#[inline]
pub fn offset_directory_with_frames(input: &[u8],
                                    offset: usize)
                                    -> IResult<&[u8], Directory, Error> {
    match offset_directory(input, offset) {
        IResult::Done(_, mut directory) => {
            for entry in &mut directory.entries {
                entry.frames = match offset_frames(input, entry.offset as usize) {
                    IResult::Done(_, frames) => frames,
                    other => return other.map(|_| unreachable!()),
                };
            }

            IResult::Done(input, directory)
        }
        other => other,
    }
}
