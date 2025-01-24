use crate::geographic::Position;

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

#[derive(Debug, Clone)]
pub struct Location {
    pub building: String,
    pub floor: String,
    pub room: String,
}

impl Location {
    pub fn new(building: &str, floor: &str, room: &str) -> Self {
        Self {
            building: building.to_owned(),
            floor: floor.to_owned(),
            room: room.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Beacon {
    pub id: Id,
    pub position: Position,
    pub location: Location,
}

impl Beacon {
    pub fn new(id: Id, position: Position, location: Location) -> Self {
        Self {
            id,
            position,
            location,
        }
    }
}
