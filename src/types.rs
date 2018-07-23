//! Types for objects which demos consist of.

use errors::*;
use parse;

/// A Goldsource demo.
#[derive(Debug, PartialEq)]
pub struct Demo<'a> {
    pub header: Header<'a>,
    pub directory: Directory<'a>,
}

impl<'a> Demo<'a> {
    /// Parses a demo.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// extern crate hldemo;
    ///
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// let mut bytes = Vec::new();
    /// let mut f = File::open("demo.dem")?;
    /// f.read_to_end(&mut bytes);
    ///
    /// let demo = hldemo::Demo::parse(&bytes)?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn parse(input: &[u8]) -> Result<Demo> {
        parse::demo(input).map(|(_, demo)| demo).map_err(Into::into)
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
    ///
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// let mut bytes = Vec::new();
    /// let mut f = File::open("demo.dem")?;
    /// f.read_to_end(&mut bytes);
    ///
    /// let demo = hldemo::Demo::parse_without_frames(&bytes)?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn parse_without_frames(input: &[u8]) -> Result<Demo> {
        parse::demo_without_frames(input).map(|(_, demo)| demo)
                                         .map_err(Into::into)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Header<'a> {
    pub demo_protocol: i32,
    pub net_protocol: i32,
    pub map_name: &'a [u8],
    pub game_dir: &'a [u8],
    pub map_crc: i32,
    pub directory_offset: i32,
}

#[derive(Debug, PartialEq)]
pub struct Directory<'a> {
    pub entries: Vec<DirectoryEntry<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct DirectoryEntry<'a> {
    pub entry_type: i32,
    pub description: &'a [u8],
    pub flags: i32,
    pub cd_track: i32,
    pub track_time: f32,
    pub frame_count: i32,
    pub offset: i32,
    pub file_length: i32,

    pub frames: Vec<Frame<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Frame<'a> {
    pub time: f32,
    pub frame: i32,
    pub data: FrameData<'a>,
}

#[derive(Debug, PartialEq)]
pub enum FrameData<'a> {
    NetMsgStart(NetMsgData<'a>),
    NetMsg(NetMsgData<'a>),
    DemoStart,
    ConsoleCommand(ConsoleCommandData<'a>),
    ClientData(ClientDataData),
    NextSection,
    Event(EventData),
    WeaponAnim(WeaponAnimData),
    Sound(SoundData<'a>),
    DemoBuffer(DemoBufferData<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConsoleCommandData<'a> {
    pub command: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct ClientDataData {
    pub origin: [f32; 3],
    pub viewangles: [f32; 3],
    pub weapon_bits: i32,
    pub fov: f32,
}

#[derive(Debug, PartialEq)]
pub struct EventData {
    pub flags: i32,
    pub index: i32,
    pub delay: f32,
    pub args: EventArgs,
}

#[derive(Debug, PartialEq)]
pub struct EventArgs {
    pub flags: i32,
    pub entity_index: i32,
    pub origin: [f32; 3],
    pub angles: [f32; 3],
    pub velocity: [f32; 3],
    pub ducking: i32,
    pub fparam1: f32,
    pub fparam2: f32,
    pub iparam1: i32,
    pub iparam2: i32,
    pub bparam1: i32,
    pub bparam2: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WeaponAnimData {
    pub anim: i32,
    pub body: i32,
}

#[derive(Debug, PartialEq)]
pub struct SoundData<'a> {
    pub channel: i32,
    pub sample: &'a [u8],
    pub attenuation: f32,
    pub volume: f32,
    pub flags: i32,
    pub pitch: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DemoBufferData<'a> {
    pub buffer: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct NetMsgData<'a> {
    pub info: NetMsgInfo<'a>,
    pub incoming_sequence: i32,
    pub incoming_acknowledged: i32,
    pub incoming_reliable_acknowledged: i32,
    pub incoming_reliable_sequence: i32,
    pub outgoing_sequence: i32,
    pub reliable_sequence: i32,
    pub last_reliable_sequence: i32,
    pub msg: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct NetMsgInfo<'a> {
    pub timestamp: f32,
    pub ref_params: RefParams,
    pub usercmd: UserCmd,
    pub movevars: MoveVars<'a>,
    pub view: [f32; 3],
    pub viewmodel: i32,
}

#[derive(Debug, PartialEq)]
pub struct RefParams {
    pub vieworg: [f32; 3],
    pub viewangles: [f32; 3],
    pub forward: [f32; 3],
    pub right: [f32; 3],
    pub up: [f32; 3],
    pub frametime: f32,
    pub time: f32,
    pub intermission: i32,
    pub paused: i32,
    pub spectator: i32,
    pub onground: i32,
    pub waterlevel: i32,
    pub simvel: [f32; 3],
    pub simorg: [f32; 3],
    pub viewheight: [f32; 3],
    pub idealpitch: f32,
    pub cl_viewangles: [f32; 3],
    pub health: i32,
    pub crosshairangle: [f32; 3],
    pub viewsize: f32,
    pub punchangle: [f32; 3],
    pub maxclients: i32,
    pub viewentity: i32,
    pub playernum: i32,
    pub max_entities: i32,
    pub demoplayback: i32,
    pub hardware: i32,
    pub smoothing: i32,
    pub ptr_cmd: i32,
    pub ptr_movevars: i32,
    pub viewport: [i32; 4usize],
    pub next_view: i32,
    pub only_client_draw: i32,
}

#[derive(Debug, PartialEq)]
pub struct UserCmd {
    pub lerp_msec: i16,
    pub msec: u8,
    pub viewangles: [f32; 3],
    pub forwardmove: f32,
    pub sidemove: f32,
    pub upmove: f32,
    pub lightlevel: i8,
    pub buttons: u16,
    pub impulse: i8,
    pub weaponselect: i8,
    pub impact_index: i32,
    pub impact_position: [f32; 3],
}

#[derive(Debug, PartialEq)]
pub struct MoveVars<'a> {
    pub gravity: f32,
    pub stopspeed: f32,
    pub maxspeed: f32,
    pub spectatormaxspeed: f32,
    pub accelerate: f32,
    pub airaccelerate: f32,
    pub wateraccelerate: f32,
    pub friction: f32,
    pub edgefriction: f32,
    pub waterfriction: f32,
    pub entgravity: f32,
    pub bounce: f32,
    pub stepsize: f32,
    pub maxvelocity: f32,
    pub zmax: f32,
    pub wave_height: f32,
    pub footsteps: i32,
    pub sky_name: &'a [u8],
    pub rollangle: f32,
    pub rollspeed: f32,
    pub skycolor_r: f32,
    pub skycolor_g: f32,
    pub skycolor_b: f32,
    pub skyvec_x: f32,
    pub skyvec_y: f32,
    pub skyvec_z: f32,
}
