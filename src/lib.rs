use crate::loadingbar::Loadingbar;
use crate::room::RoomId;

use rayon::prelude::*;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;
use std::u32;

use std::sync::{Arc, Mutex};

mod free;
mod icalparser;
mod loadingbar;

mod room;
use room::calc_distance;

const COURSES_FILE: &str = "courses.json";

pub async fn get_rooms(
    reload: bool,
    roomname: &str,
    room_count: usize,
) -> Result<Vec<(String, u32)>, Box<dyn std::error::Error>> {
    if let Some(destination_room) = RoomId::from_str(roomname) {
        if !Path::new(COURSES_FILE).exists() || reload {
            let body = get_courses().await;
            match body {
                Ok(text) => {
                    write_file(text).expect("Error writing file");
                }
                Err(e) => eprintln!("Error: {e}"),
            }
            let json_str =
                fs::read_to_string(COURSES_FILE).expect("Should have been able to read the file");
            let json: Value = serde_json::from_str(&json_str)?;
            fs::create_dir_all("courses")?;
            let courses = json.as_array().unwrap();

            let mut bar = Loadingbar::new("Loading calendars", courses.iter().len());

            for coursename in courses {
                let name = coursename.as_str().unwrap();

                bar.print(&format!("Downloading: {}.ics", name));
                let _ = download_with_retry(&name, 3).await;

                bar.next();
            }

            println!();

            icalparser::parse_all_calendars()?;
        }
        let paths: Vec<_> = match fs::read_dir("rooms") {
            Ok(files) => files.collect(),
            Err(e) => {
                eprintln!("Error while reading rooms: {}", e);
                std::process::exit(1);
            }
        };

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
        println!();
        Ok(min_keys[0..room_count].to_vec())
    } else {
        return Err(format!("{} is not a valid roomname", roomname).into());
    }
}
async fn get_courses() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://api.dhbw.app/courses/KA/")
        .await?
        .text()
        .await?;
    Ok(body)
}

fn write_file(contents: String) -> io::Result<()> {
    let mut file = File::create(COURSES_FILE)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

async fn download_with_retry(
    name: &str,
    max_retries: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://dhbw.app/ical/{}", name);
    let mut attempts = 0;

    loop {
        attempts += 1;
        let response = reqwest::get(&url).await;

        match response {
            Ok(resp) => match resp.error_for_status() {
                Ok(success_resp) => {
                    let bytes = success_resp.bytes().await?;
                    let mut out = std::fs::File::create(format!("courses/{}.ics", name))?;
                    out.write_all(&bytes)?;
                    return Ok(());
                }
                Err(e) => {
                    print!("\rHTTP error (attempt {}): {}\n", attempts, e);
                }
            },
            Err(e) => {
                print!("\rRequest failed (attempt {}): {}\n", attempts, e);
            }
        }

        if attempts >= max_retries {
            return Err("Max retries reached".into());
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
