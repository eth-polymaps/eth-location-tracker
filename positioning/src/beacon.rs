#[derive(Debug, Clone)]
pub struct Id {
    pub uuid: String,
    pub major: u16,
    pub minor: u16,
}

impl Id {
    pub fn new(uuid: &str, major: u16, minor: u16) -> Self {
        Id {
            uuid: uuid.to_string(),
            major,
            minor,
        }
    }
}
