use crate::geographic::Position;

#[derive(Debug, Clone)]
pub struct BeaconId {
    pub uuid: String,
    pub major: u16,
    pub minor: u16,
}
impl BeaconId {
    pub fn new(uuid: &str, major: u16, minor: u16) -> Self {
        BeaconId {
            uuid: uuid.to_string(),
            major,
            minor,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Beacon {
    pub id: BeaconId,
    pub location: Room,
    pub position: Position,
}

impl Beacon {
    pub fn new(id: BeaconId, location: Room, position: Position) -> Self {
        Self {
            id,
            location,
            position,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Room {
    pub building: String,
    pub floor: String,
    pub room: String,
}

impl Room {
    pub fn identifier(&self) -> String {
        format!("{}/{}/{}", self.building, self.floor, self.room)
    }

    pub fn new(building: &str, floor: &str, room: &str) -> Self {
        Self {
            building: building.to_owned(),
            floor: floor.to_owned(),
            room: room.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Output {
    pub position: Position,
    pub location: Room,
    pub speed: Option<f32>,
    pub heading: Option<i32>,
}

impl Output {
    pub fn new(
        position: Position,
        location: Room,
        speed: Option<f32>,
        heading: Option<i32>,
    ) -> Self {
        Self {
            position,
            location,
            speed,
            heading,
        }
    }
}
