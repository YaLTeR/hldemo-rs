use nom::*;

use super::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub demo_buffer_data<FrameData>,
    map!(length_bytes!(le_i32), |buffer| FrameData::DemoBuffer(DemoBufferData { buffer }))
);
