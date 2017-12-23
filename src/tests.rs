use super::*;

#[test]
fn basic() {
    let bytes = include_bytes!("../test-demos/basic.dem");
    let demo = Demo::parse(bytes).unwrap();

    // Header
    assert_eq!(demo.header.demo_protocol, 5);
    assert_eq!(demo.header.net_protocol, 48);

    let mut map_name = [0; 260];
    map_name[..4].copy_from_slice(b"c1a0");
    assert_eq!(demo.header.map_name, &map_name[..]);

    let mut game_dir = [0; 260];
    game_dir[..5].copy_from_slice(b"valve");
    assert_eq!(demo.header.game_dir, &game_dir[..]);

    assert_eq!(demo.header.map_crc, 0);
    assert_eq!(demo.header.directory_offset, 193192);

    // Directory
    assert_eq!(demo.directory.entries.len(), 2);

    // Entry 1
    let entry = &demo.directory.entries[0];
    assert_eq!(entry.entry_type, 0);

    let mut description = [0; 64];
    description[..7].copy_from_slice(b"LOADING");
    assert_eq!(entry.description, &description[..]);

    assert_eq!(entry.flags, 0);
    assert_eq!(entry.cd_track, -1);
    assert_eq!(entry.track_time, 0f32);
    assert_eq!(entry.frame_count, 0);
    assert_eq!(entry.offset, 544);
    assert_eq!(entry.file_length, 25268);

    assert_eq!(entry.frames.len(), 6);

    // Entry 2
    let entry = &demo.directory.entries[1];
    assert_eq!(entry.entry_type, 1);

    let mut description = [0; 64];
    description[..8].copy_from_slice(b"Playback");
    assert_eq!(entry.description, &description[..]);

    assert_eq!(entry.flags, 0);
    assert_eq!(entry.cd_track, -1);
    // Not sure how good of an idea this one is, but it seems to work.
    assert_eq!(entry.track_time, 2.8860817f32);
    assert_eq!(entry.frame_count, 289);
    assert_eq!(entry.offset, 25812);
    assert_eq!(entry.file_length, 167380);

    assert_eq!(entry.frames.len(), 911);
}

#[test]
fn without_frames() {
    let bytes = include_bytes!("../test-demos/basic.dem");
    let demo = Demo::parse_without_frames(bytes).unwrap();

    assert!(demo.directory.entries[0].frames.is_empty());
    assert!(demo.directory.entries[1].frames.is_empty());
}

#[test]
fn frame_types() {
    let bytes = include_bytes!("../test-demos/frame-types.dem");
    let demo = Demo::parse(bytes).unwrap();
    let frames = &demo.directory.entries[0].frames;

    assert!(if let FrameData::NetMsgStart(_) = frames[0].data {
                true
            } else {
                false
            });
    assert!(if let FrameData::NetMsg(_) = frames[1].data {
                true
            } else {
                false
            });
    assert!(if let FrameData::DemoStart = frames[2].data {
                true
            } else {
                false
            });

    let mut command = [0; 64];
    command[..11].copy_from_slice(b"hello world");
    assert!(if let FrameData::ConsoleCommand(ConsoleCommandData { command: c }) = frames[3].data {
                c == &command[..]
            } else {
                false
            });

    assert!(if let FrameData::ClientData(_) = frames[4].data {
                true
            } else {
                false
            });
    assert!(if let FrameData::Event(_) = frames[5].data {
                true
            } else {
                false
            });
    assert!(if let FrameData::WeaponAnim(_) = frames[6].data {
                true
            } else {
                false
            });
    assert!(if let FrameData::Sound(_) = frames[7].data {
                true
            } else {
                false
            });
    assert!(if let FrameData::DemoBuffer(_) = frames[8].data {
                true
            } else {
                false
            });
    assert!(if let FrameData::NextSection = frames[9].data {
                true
            } else {
                false
            });
}

#[test]
fn error_invalid_magic() {
    let bytes = include_bytes!("../test-demos/invalid-magic.dem");
    let error = Demo::parse_without_frames(bytes).err().unwrap();
    let mut error_iter = error.iter();

    // Can't downcast errors. :(
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::Header));
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::InvalidMagic));
}

#[test]
fn error_invalid_demo_protocol() {
    let bytes = include_bytes!("../test-demos/invalid-demo-protocol.dem");
    let error = Demo::parse_without_frames(bytes).err().unwrap();
    let mut error_iter = error.iter();

    // Can't downcast errors. :(
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::Header));
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::InvalidDemoProtocol(4)));
}

#[test]
fn error_invalid_directory_entry_count() {
    let bytes = include_bytes!("../test-demos/invalid-directory-entry-count.dem");
    let error = Demo::parse_without_frames(bytes).err().unwrap();
    let mut error_iter = error.iter();

    // Can't downcast errors. :(
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::Directory));
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::InvalidDirectoryEntryCount(65535)));
}

#[test]
fn error_invalid_frame_type() {
    let bytes = include_bytes!("../test-demos/invalid-frame-type.dem");
    let error = Demo::parse(bytes).err().unwrap();
    let mut error_iter = error.iter();

    // Can't downcast errors. :(
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::Frames));
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::InvalidFrameType(10)));
}

#[test]
fn error_invalid_netmsg_length() {
    let bytes = include_bytes!("../test-demos/invalid-netmsg-length.dem");
    let error = Demo::parse(bytes).err().unwrap();
    let mut error_iter = error.iter();

    // Can't downcast errors. :(
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::Frames));
    assert_eq!(format!("{}", error_iter.next().unwrap()),
               format!("{}", parse::Error::InvalidNetMsgLength(16777215)));
}
