use std::{fs::File, io::BufReader};

mod file;
mod version;
mod frame;
mod header;
mod flags;

pub fn parse_final(replay: BufReader<File>) -> () {

}