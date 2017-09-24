use super::*;
use super::directory::*;
use super::header::*;
use types::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub demo_without_frames<&[u8], Demo, Error>,
    do_parse!(
        header:    peek!(header)                                             >>
        directory: call!(offset_directory, header.directory_offset as usize) >>
        (
            Demo {
                header,
                directory,
            }
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub demo<&[u8], Demo, Error>,
    do_parse!(
        header:    peek!(header)                                                         >>
        directory: call!(offset_directory_with_frames, header.directory_offset as usize) >>
        (
            Demo {
                header,
                directory,
            }
        )
    )
);
