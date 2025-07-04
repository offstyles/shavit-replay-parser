use std::{fs::File, io::{BufRead, BufReader, Read}};

use crate::errors::ReplayParsingError;
use super::{
    flags::{ButtonFlags, EntityFlags, MoveType},
    header::ReplayHeaderFinal,
};

#[derive(Debug)]
pub struct ReplayFrameFinal {
    pos: [f32; 3],
    ang: [f32; 2],
    buttons: ButtonFlags,
    
    // only if subversion is >= 0x02
    flags: Option<EntityFlags>,
    mt: Option<MoveType>,

    // only if subversion is >= 0x06
    mouse_xy: Option<u32>, // `mousex | (mousey << 16)` // unpack with UnpackSignedShorts
    velocity: Option<u32>, // basically `forwardmove | (sidemove << 16)` // unpack with UnpackSignedShorts
}

pub fn parse_replay_frames(reader: &mut BufReader<File>, header: &ReplayHeaderFinal) -> Result<Vec<ReplayFrameFinal>, ReplayParsingError> {
    // The cells are the amount of 4 byte sections in a replay frame
    // 6 is the default for old replays
    let mut cells: u32 = 6;

    if header.get_subversion() > &0x01u8 {
        cells = 8;
    }

    if header.get_subversion() >= &0x06u8 {
        cells = 10;
    }

    unimplemented!()
}
