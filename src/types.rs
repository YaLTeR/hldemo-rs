#[derive(Debug)]
pub struct Demo<'a> {
    pub header: Header<'a>,
    pub directory: Directory<'a>,
}

#[derive(Debug)]
pub struct Header<'a> {
    pub demo_protocol: i32,
    pub net_protocol: i32,
    pub map_name: &'a str,
    pub game_dir: &'a str,
    pub map_crc: i32,
    pub directory_offset: i32,
}

#[derive(Debug)]
pub struct Directory<'a> {
    pub entries: Vec<DirectoryEntry<'a>>,
}

#[derive(Debug)]
pub struct DirectoryEntry<'a> {
    pub entry_type: i32,
    pub description: &'a str,
    pub flags: i32,
    pub cd_track: i32,
    pub track_time: f32,
    pub frame_count: i32,
    pub offset: i32,
    pub file_length: i32,
}
