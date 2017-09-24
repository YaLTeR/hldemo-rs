use super::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub console_command_data<FrameData>,
    map!(take!(64), |command| FrameData::ConsoleCommand(ConsoleCommandData { command }))
);
