#[derive(Debug)]
pub struct IBeaconData {
    pub uuid: String,
    pub major: u16,
    pub minor: u16,
    pub power: i8,
}

pub fn from_bytes(payload: &[u8]) -> Option<IBeaconData> {
    const IBEACON_PREFIX: [u8; 4] = [0x4c, 0x00, 0x02, 0x15];

    if payload.is_empty() {
        return None;
    }

    // Find iBeacon prefix in the payload
    let mut i = 0;
    while i < payload.len() - 4 {
        if payload[i..i + 4] == IBEACON_PREFIX {
            // Found iBeacon prefix, now parse the data
            let start_idx = i + 4;

            // Ensure we have enough bytes remaining
            if start_idx + 20 > payload.len() {
                return None;
            }

            // Extract UUID (16 bytes)
            let mut uuid = [0u8; 16];
            uuid.copy_from_slice(&payload[start_idx..start_idx + 16]);

            // Extract major (2 bytes, big endian)
            let major = u16::from_be_bytes([payload[start_idx + 16], payload[start_idx + 17]]);

            // Extract minor (2 bytes, big endian)
            let minor = u16::from_be_bytes([payload[start_idx + 18], payload[start_idx + 19]]);

            // Extract power (1 byte, signed)
            let power = payload[start_idx + 20] as i8;

            return Some(IBeaconData {
                uuid: format_uuid(&uuid),
                major,
                minor,
                power,
            });
        }
        i += 1;
    }
    None
}

fn format_uuid(uuid: &[u8; 16]) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid[0],
        uuid[1],
        uuid[2],
        uuid[3],
        uuid[4],
        uuid[5],
        uuid[6],
        uuid[7],
        uuid[8],
        uuid[9],
        uuid[10],
        uuid[11],
        uuid[12],
        uuid[13],
        uuid[14],
        uuid[15]
    )
}
