
use super::version::ReplayVersion;
use super::frame::ReplayFrameFinal;

#[derive(Debug)]
pub struct ReplayFile<'a> {
    version: ReplayVersion,
    map: Option<&'a str>,
    style: Option<u8>,
    track: Option<u8>,
    pre_frames: Option<u32>,
    frame_count: u32,
    post_frames: Option<u32>,
    time: f32,
    steam_id: u64,
    tickrate: Option<u16>,
    zone_offset: Option<[f32; 2]>,

    frame_data: ReplayFrameFinal,
}