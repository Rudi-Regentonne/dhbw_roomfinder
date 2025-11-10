use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomId {
    pub block: char,
    pub floor: u8,
    pub number: u16,
}

impl RoomId {
    pub fn from_str(s: &str) -> Option<Self> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 3 {
            return None;
        }
        let block = chars[0].to_ascii_uppercase();
        let floor = chars[1].to_digit(10)? as u8;
        let number = s[2..].parse().ok()?;
        Some(RoomId {
            block,
            floor,
            number,
        })
    }
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        return format!("{}{}{}", self.block, self.floor, self.number);
    }
}
#[allow(dead_code)]
pub fn calc_distance(destination: &RoomId, room: &str) -> u32 {
    if let Some(room_id) = RoomId::from_str(&room) {
        let distance = ((room_id.block as i32 - destination.block as i32).abs() * 1000
            + (room_id.floor as i32 - destination.floor as i32).abs() * 100
            + (room_id.number as i32 - destination.number as i32).abs())
            as u32;

        return distance;
    }
    return u32::MAX;
}
