#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,

    pub minor: u32,

    pub patch: u32,
}

impl Version {
    pub fn parse(value: &str) -> Self {
        let parts: Vec<u32> = value.split('.').map(|x| x.parse().unwrap_or(0)).collect();

        Self {
            major: *parts.get(0).unwrap_or(&0),

            minor: *parts.get(1).unwrap_or(&0),

            patch: *parts.get(2).unwrap_or(&0),
        }
    }
}

pub fn compatible(current: &Version, required: &Version) -> bool {
    current >= required
}
