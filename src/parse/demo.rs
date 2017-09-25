use super::*;
use super::directory::*;
use super::header::*;
use types::*;

/// Parses a demo.
pub fn demo(input: &[u8]) -> IResult<&[u8], Demo, Error> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(input,
        header:    peek!(header)                                                         >>
        directory: call!(offset_directory_with_frames, header.directory_offset as usize) >>
        (
            Demo {
                header,
                directory,
            }
        )
    )
}

/// Parses a demo's header and directory, without parsing frame data.
///
/// Parsing frames usually takes a long time, so this function can be used when the frame data
/// isn't needed.
pub fn demo_without_frames(input: &[u8]) -> IResult<&[u8], Demo, Error> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(input,
        header:    peek!(header)                                             >>
        directory: call!(offset_directory, header.directory_offset as usize) >>
        (
            Demo {
                header,
                directory,
            }
        )
    )
}
