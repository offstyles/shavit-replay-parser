use std::{fs::File, io::{BufRead, BufReader, Read}};

use crate::errors::ReplayParsingError;
use super::{
    flags::{ButtonFlags, EntityFlags, MoveType},
    header::ReplayHeaderFinal,
};
use crate::read_to_num;

#[derive(Debug)]
pub struct ReplayFrameFinal {
    pos: [f32; 3],
    ang: [f32; 2],
    buttons: ButtonFlags,
    
    // only if subversion is >= 0x02
    flags: Option<EntityFlags>,
    mt: Option<MoveType>,

    // only if subversion is >= 0x06
    mouse: Option<[i16; 2]>,
    velocity: Option<[i16; 2]>,
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

    let total = header.get_total_frames().max(0) as usize;
    let mut frames: Vec<ReplayFrameFinal> = Vec::with_capacity(total);

    for _ in 0..total {
        let pos = [
            read_to_num!(reader, 4, f32),
            read_to_num!(reader, 4, f32),
            read_to_num!(reader, 4, f32),
        ];
        let ang = [
            read_to_num!(reader, 4, f32),
            read_to_num!(reader, 4, f32),
        ];

        // dont have to retain here because we mark unknown bits as UNUSEDX but might aswell
        let buttons = ButtonFlags::from_bits_retain(read_to_num!(reader, 4, u32));

        let (flags, mt) = if cells >= 8 {
            let flags = EntityFlags::from_bits_retain(read_to_num!(reader, 4, u32));
            let mt = MoveType::try_from(read_to_num!(reader, 4, i32))?;
            (Some(flags), Some(mt))
        } else {
            (None, None)
        };

        let (mouse, velocity) = if cells >= 10 {
            // the replay format packs 2 i16s into an i32, but we can just read 2 i16s
            // mousex | (mousey << 16) and forwardmove | (sidemove << 16)
            let mouse = [read_to_num!(reader, 2, i16), read_to_num!(reader, 2, i16)];
            let velocity = [read_to_num!(reader, 2, i16), read_to_num!(reader, 2, i16)];
            (Some(mouse), Some(velocity))
        } else {
            (None, None)
        };

        frames.push(ReplayFrameFinal {
            pos,
            ang,
            buttons,
            flags,
            mt,
            mouse,
            velocity,
        });
    }

    Ok(frames)
}
