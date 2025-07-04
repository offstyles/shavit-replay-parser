#[derive(Debug)]
pub(crate) enum ReplayVersion {
    Unknown,
    V2(u8),
    Final(u8)
}

impl ReplayVersion {
    fn from(value: &str, subver: u8) -> Self {
        match value {
            "{SHAVITREPLAYFORMAT}{V2}" => ReplayVersion::V2(subver),
            "{SHAVITREPLAYFORMAT}{FINAL}" => ReplayVersion::Final(subver),
            _ => ReplayVersion::Unknown
        }
    }
}
