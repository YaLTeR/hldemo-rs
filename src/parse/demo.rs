use super::*;
use super::directory::*;
use super::header::*;
use types::*;

/// Parses a demo.
///
/// # Examples
///
/// ```no_run
/// # use std::error::Error;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// extern crate hldemo;
/// extern crate nom;
///
/// use std::fs::File;
/// use std::io::Read;
///
/// let mut bytes = Vec::new();
/// let mut f = File::open("demo.dem")?;
/// f.read_to_end(&mut bytes);
///
/// let demo = match hldemo::parse::demo(&bytes) {
///     nom::IResult::Done(_, demo) => Ok(demo),
///     _ => Err("need more bytes or parsing failure"),
/// }?;
/// #
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
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
///
/// # Examples
///
/// ```no_run
/// # use std::error::Error;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// extern crate hldemo;
/// extern crate nom;
///
/// use std::fs::File;
/// use std::io::Read;
///
/// let mut bytes = Vec::new();
/// let mut f = File::open("demo.dem")?;
/// f.read_to_end(&mut bytes);
///
/// let demo = match hldemo::parse::demo_without_frames(&bytes) {
///     nom::IResult::Done(_, demo) => Ok(demo),
///     _ => Err("need more bytes or parsing failure"),
/// }?;
/// #
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
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
