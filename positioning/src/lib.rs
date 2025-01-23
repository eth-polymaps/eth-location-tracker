pub mod buffer;
pub mod location;
pub mod signal;

#[derive(Debug, Clone)]
pub struct Beacon {
    pub uuid: String,
    pub major: u16,
    pub minor: u16,
}

impl Beacon {
    pub fn new(uuid: &str, major: u16, minor: u16) -> Self {
        Beacon {
            uuid: uuid.to_string(),
            major,
            minor,
        }
    }
}
