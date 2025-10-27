use rayon::prelude::*;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::u32;

use crate::loadingbar::Loadingbar;

use std::sync::{Arc, Mutex};

mod free;
mod icalparser;
mod loadingbar;

struct RoomId {
    block: char,
    floor: u8,
    number: u16,
}

impl RoomId {
    fn from_str(s: &str) -> Option<Self> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 3 {
            return None;
        }
        let block = chars[0];
        let floor = chars[1].to_digit(10)? as u8;
        let number = s[2..].parse().ok()?;
        Some(RoomId {
            block,
            floor,
            number,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("data.json").exists() {
        let body = get_json().await;
        match body {
            Ok(text) => {
                write_file(text).expect("Error writing file");
            }
            Err(e) => println!("Error: {e}"),
        }
    }
    let json_str = fs::read_to_string("data.json").expect("Should have been able to read the file");
    let json: Value = serde_json::from_str(&json_str)?;
    let mut bar = Loadingbar::new("Loading calendars", json.as_array().unwrap().len());
    fs::create_dir_all("courses")?;
    for coursename in json.as_array().unwrap() {
        let name = &coursename.to_string()[1..coursename.to_string().len() - 1];
        if !Path::new(&(format!("courses/{}.ics", name))).exists() {
            bar.print(&format!("Downloading: {}.ics", name));
            let _ = download(&name).await;
        } else {
            bar.print(&format!("File exists for: {}", name));
        }
        bar.next();
    }
    println!();
    icalparser::parse_all_calendars()?;
    let destination_room = RoomId::from_str("A266").unwrap();
    let paths: Vec<_> = fs::read_dir("rooms").unwrap().collect();
    let bar = Arc::new(Mutex::new(Loadingbar::new(
        "Finding rooms",
        paths.iter().len(),
    )));
    let mut min_keys: Vec<(String, u32)> = paths
        .par_iter()
        .map(|roomname| {
            let roomname = roomname
                .as_ref()
                .unwrap()
                .path()
                .display()
                .to_string()
                .replace(".ics", "")
                .replace("rooms/", "");
            if free::is_free(&roomname) {
                let new_distance = calc_distance(&destination_room, &roomname);
                let mut bar = bar.lock().unwrap();
                bar.next();
                return (roomname, new_distance);
            }
            let mut bar = bar.lock().unwrap();
            bar.next();
            (roomname, u32::MAX)
        })
        .collect();
    min_keys.sort_by_key(|(_, dist)| *dist);
    for (roomname, distance) in min_keys.iter().take(5) {
        println!("{} (distance: {})", roomname, distance);
    }

    Ok(())
}

fn calc_distance(destination: &RoomId, room: &str) -> u32 {
    if let Some(room_id) = RoomId::from_str(&room) {
        let distance = ((room_id.block as i32 - destination.block as i32).abs() * 1000
            + (room_id.floor as i32 - destination.floor as i32).abs() * 100
            + (room_id.number as i32 - destination.number as i32).abs())
            as u32;

        return distance;
    }
    return u32::MAX;
}

async fn get_json() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://api.dhbw.app/courses/KA/")
        .await?
        .text()
        .await?;
    Ok(body)
}

fn write_file(contents: String) -> io::Result<()> {
    let mut file = File::create("data.json")?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
async fn download(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://dhbw.app/ical/{}", name);
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let mut out = File::create(format!("courses/{}.ics", name))?;
    out.write_all(&bytes)?;
    Ok(())
}
