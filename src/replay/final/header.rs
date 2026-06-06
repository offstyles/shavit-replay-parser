use std::{fs::File, io::{BufRead, BufReader, Read}};

use crate::errors::ReplayParsingError;
use crate::replay::steamid3_to_steamid64;
use crate::{read_to_string, read_to_num};

#[derive(Debug)]
pub(crate) struct ReplayHeaderFinal {
    pub minor_version: u8,
    map: Option<String>,
    style: Option<u8>,
    track: Option<u8>,
    pre_frames: i32,
    frame_count: i32,
    post_frames: i32,
    time: Option<f32>,
    steam_id: Option<u64>,
    tickrate: Option<f32>,
    zone_offset: Option<[f32;2]>,
    timestamp: Option<u32>,
}

impl ReplayHeaderFinal {
    pub fn get_subversion(&self) -> &u8 {
        &self.minor_version
    }

    pub fn get_total_frames(&self) -> i32 {
        &self.frame_count + &self.post_frames + &self.pre_frames
    }
}

pub fn parse_final_header(reader: &mut BufReader<File>, version: &u8) -> Result<ReplayHeaderFinal, ReplayParsingError> {
    let mut map: Option<String> = None;
    let mut style: Option<u8> = None;
    let mut track: Option<u8> = None;
    let mut pre_frames: i32;
    let mut frame_count: i32;
    let mut post_frames: i32;
    let mut time: Option<f32> = None;
    let mut steamid_64: Option<u64> = None;
    let mut tickrate: Option<f32> = None;
    let mut zone_offset: Option<[f32; 2]> = None;
    let mut timestamp: Option<u32> = None;
    
    if version >= &0x03u8 {
        let mut map_buf: Vec<u8> = vec![];
        let _ = reader.read_until(b'\0', &mut map_buf)?;
        map = Some(String::from_utf8(map_buf)?);
        
        let mut style_buf: [u8; 1] = [0; 1];
        let _ = reader.read_exact(&mut style_buf)?;
        style = Some(u8::from_le_bytes(style_buf));

        let mut track_buf: [u8; 1] = [0; 1];
        let _ = reader.read_exact(&mut track_buf)?;
        track = Some(u8::from_le_bytes(track_buf));

        let mut preframe_buf: [u8; 4] = [0; 4];
        let _ = reader.read_exact(&mut preframe_buf)?;

        // i hope this works correctly, not sure if sourcemod writes bytes as le or be,
        // so lets just throw up le and see what happens...
        pre_frames = i32::from_le_bytes(preframe_buf);
        if pre_frames < 0 {
            pre_frames = 0;
        }
    } else {
        // original replay code doesnt do this, i wonder if that results in broken replays
        // on versions less than 0x03
        pre_frames = 0;
    };

    let mut framecount_buf: [u8; 4] = [0; 4];
    let _ = reader.read_exact(&mut framecount_buf)?;
    frame_count = i32::from_le_bytes(framecount_buf);

    let mut time_buf: [u8; 4] = [0; 4];
    let _ = reader.read_exact(&mut time_buf)?;
    time = Some(f32::from_le_bytes(time_buf));

    if version >= &0x04u8 {
        // let mut steamid_buf: [u8; 4] = [0; 4];
        // let _ = reader.read_exact(&mut steamid_buf)?;
        // let steamid3 = u32::from_le_bytes(steamid_buf);
        let steamid3 = read_to_num!(reader, 4, u32);
        steamid_64 = Some(steamid3_to_steamid64(format!("[U:1:{}]", steamid3))
            .ok_or(ReplayParsingError::InvalidSteamID(format!("[U:1:{}]", steamid3)))?);
    } else {
        // Old versions have steamid formatted as [U:1:XXXXXX] 
        // while new versions only have the ending
        // let mut steamid3_buf: Vec<u8> = vec![];
        // let _ = reader.read_until(b'\0', &mut steamid3_buf)?;
        // let steamid3 = String::from_utf8(steamid3_buf)?;
        let steamid3 = read_to_string!(reader, b'\0');
        steamid_64 = Some(steamid3_to_steamid64(&steamid3)
            .ok_or(ReplayParsingError::InvalidSteamID(steamid3))?);
    }

    if version >= &0x05u8 {
        let mut postframes_buf: [u8; 4] = [0; 4];
        let _ = reader.read_exact(&mut postframes_buf)?;
        post_frames = i32::from_le_bytes(postframes_buf);

        if post_frames < 0 {
            post_frames = 0;
        }

        // let mut tickrate_buf: [u8; 4] = [0; 4];
        // let _ = reader.read_exact(&mut tickrate_buf)?;
        // tickrate = Some(f32::from_le_bytes(tickrate_buf));
        tickrate = Some(read_to_num!(reader, 4, f32));
    } else {
        post_frames = 0;
    }

    if version < &0x07u8 {
        frame_count -= pre_frames;
        frame_count -= post_frames;
    }

    if version >= &0x08u8 {
        let mut zone_offset1: [u8; 4] = [0; 4];
        let mut zone_offset2: [u8; 4] = [0; 4];
        let _ = reader.read_exact(&mut zone_offset1)?;
        let _ = reader.read_exact(&mut zone_offset2)?;
        
        zone_offset = Some([f32::from_le_bytes(zone_offset1), f32::from_le_bytes(zone_offset2)]);
    }

    // included in 0x0c in the header
    if version >= &0x0cu8 {
        timestamp = Some(read_to_num!(reader, 4, u32))
    }

    Ok(ReplayHeaderFinal {
        minor_version: *version,
        map: map,
        style,
        track,
        pre_frames,
        frame_count,
        post_frames,
        time,
        steam_id: steamid_64,
        tickrate,
        zone_offset,
        timestamp
    })
}

#[macro_export]
macro_rules! read_to_num {
    ($reader:ident, $size:expr, $totype:ty) => {{
        let mut ___buf: [u8; $size] = [0; $size];
        let _ = $reader.read_exact(&mut ___buf)?;

        <$totype>::from_le_bytes(___buf)
    }};
}

#[macro_export]
macro_rules! read_to_string {
    ($reader:ident, $seperator:expr) => {{
        let mut ___buf: Vec<u8> = vec![];
        let _ = $reader.read_until($seperator, &mut ___buf)?;

        String::from_utf8(___buf)?
    }};
}

use {read_to_num, read_to_string};
