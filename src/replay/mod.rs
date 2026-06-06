pub mod r#final;

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

// Util functions, probably dont need to use unpack_signed_shorts since we can just read 2 i16s instead of an i32
pub fn unpack_signed_shorts(x: i32) -> [i16; 2] {
    let mut out: [i16; 2] = [0, 0];

    out[0] = (((x & 0xFFFF) ^ 0x8000) - 0x8000) as i16;
    out[1] = ((((x >> 16) & 0x8000) ^ 0x8000) - 0x8000) as i16;

    out
}

pub fn steamid3_to_steamid64(steamid3: impl Into<String>) -> Option<u64> {
    const STEAMID64_MAGIC_NUMBER: u64 = 76561197960265728;

    let steamid3: String = steamid3.into();
    let id = steamid3
        .trim_matches(|c| c == '[' || c == ']')
        .split(":")
        .last()?
        .parse::<u64>()
        .ok()?;

    Some(id + STEAMID64_MAGIC_NUMBER)
}
