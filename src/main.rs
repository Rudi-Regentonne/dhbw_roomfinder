use chrono::{Duration, Utc};
use rayon::prelude::*;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::u32;

use crate::loadingbar::Loadingbar;

use std::sync::{Arc, Mutex};

mod config;
mod free;
mod icalparser;
mod loadingbar;
use clap::Parser;
use config::Config;

mod room;
use room::calc_distance;

const COURSES_FILE: &str = "courses.json";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'r', long = "room")]
    room: Option<String>,
    #[arg(short = 'f', long = "refetch")]
    refetch: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut config = Config::get_config(args.room)?;
    let reload = config.last_updated < Utc::now() - Duration::days(1) || args.refetch;
    if !Path::new(COURSES_FILE).exists() || reload {
        let body = get_courses().await;
        match body {
            Ok(text) => {
                write_file(text).expect("Error writing file");
                config.last_updated = Utc::now();
                let _ = config.save();
            }
            Err(e) => println!("Error: {e}"),
        }
        let json_str =
            fs::read_to_string(COURSES_FILE).expect("Should have been able to read the file");
        let json: Value = serde_json::from_str(&json_str)?;
        let mut bar = Loadingbar::new("Loading calendars", json.as_array().unwrap().len());
        fs::create_dir_all("courses")?;
        for coursename in json.as_array().unwrap() {
            let name = &coursename.to_string()[1..coursename.to_string().len() - 1];

            bar.print(&format!("Downloading: {}.ics", name));
            let _ = download(&name).await;

            bar.next();
        }
        println!();

        icalparser::parse_all_calendars()?;
    }
    //let destination_room = RoomId::from_str("A266").unwrap();
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
                let new_distance = calc_distance(&config.room, &roomname);
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
    println!("neares rooms from {} are: ", config.room.to_string());
    for (roomname, distance) in min_keys.iter().take(5) {
        println!("{} (distance: {})", roomname, distance);
    }

    Ok(())
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
async fn download(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://dhbw.app/ical/{}", name);
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let mut out = File::create(format!("courses/{}.ics", name))?;
    out.write_all(&bytes)?;
    Ok(())
}
