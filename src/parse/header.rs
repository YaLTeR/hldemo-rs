//! Demo header parsing functions.

use nom::*;

use super::*;
use types::*;

pub const SUPPORTED_DEMO_PROTOCOL: i32 = 5;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub magic<&[u8], (), Error>,
    add_parse_error!(InvalidMagic,
        fix_error!(Error, do_parse!(tag!("HLDEMO") >> take!(2) >> ()))
    )
);

#[inline]
fn check_demo_protocol(protocol: i32) -> Result<i32, Error> {
    if protocol == SUPPORTED_DEMO_PROTOCOL {
        Ok(protocol)
    } else {
        Err(Error::InvalidDemoProtocol(protocol))
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub demo_protocol<&[u8], i32, Error>,
    map_res_err_!(fix_error!(Error, le_i32), check_demo_protocol)
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub header<&[u8], Header, Error>,
    add_parse_error!(Header,
        do_parse!(
                              magic                         >>
            demo_protocol:    demo_protocol                 >>
            net_protocol:     fix_error!(Error, le_i32)     >>
            map_name:         fix_error!(Error, take!(260)) >>
            game_dir:         fix_error!(Error, take!(260)) >>
            map_crc:          fix_error!(Error, le_u32)     >>
            directory_offset: fix_error!(Error, le_i32)     >>
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
    )
);
