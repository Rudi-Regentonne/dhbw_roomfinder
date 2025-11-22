use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
mod config;
use clap::Parser;
use config::Config;
use dhbw_roomfinder::get_rooms;

/// Parses a string into a NaiveDate using either "YYYY-MM-DD" or "DD.MM.YYYY" formats.
/// Returns an error string if neither format matches.
fn parse_date(src: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(src, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(src, "%d.%m.%Y"))
        .map_err(|_| format!("invalid date format: {}", src))
}

/// Parses a string into a NaiveTime using "HH:MM" format.
/// Returns an error string if the format does not match.
fn parse_time(src: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(src, "%H:%M").map_err(|_| format!("invalid time: {}", src))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Structure for parsing and holding command-line arguments for room search.
struct Args {
    #[arg(short = 'r', long = "room")]
    room: Option<String>,
    #[arg(short = 'f', long = "refetch")]
    refetch: bool,
    #[arg(short = 'd', long = "day", value_parser = parse_date)]
    date: Option<NaiveDate>,
    #[arg(short = 't', long = "startTime", value_parser = parse_time)]
    time: Option<NaiveTime>,
    #[arg(short = 'e', long = "endTime", value_parser = parse_time)]
    end_time: Option<NaiveTime>,
}
mod room;

/// Main asynchronous entry point for the application.
/// Handles argument parsing, configuration, optional data reloads,
/// determines the time to search for rooms, and prints results.
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut config = Config::get_config(args.room).expect("failed to get config");

    // Decide whether to reload data (forced or older than 1 day)
    let reload = config.last_updated < Utc::now() - Duration::days(1) || args.refetch;
    if reload {
        config.last_updated = Utc::now();
        let _ = config.save();
    }

    // Default to current local datetime
    let mut start_time = Local::now().naive_local();
    let mut enddatetime = (start_time.date() + Duration::days(1))
        .and_hms_opt(0, 0, 0)
        .unwrap();
    println!(
        "{},{};{},{}",
        start_time.date(),
        start_time.time(),
        enddatetime.date(),
        enddatetime.time()
    );

    // Apply user-specified date, if any
    if let Some(date) = args.date {
        println!("{}", date);
        start_time = NaiveDateTime::new(date, start_time.time());
    }
    // Apply user-specified time, if any
    if let Some(time) = args.time {
        println!("{}", time);
        start_time = NaiveDateTime::new(start_time.date(), time);
    }
    if let Some(time) = args.end_time {
        println!("{}", time);
        enddatetime = NaiveDateTime::new(start_time.date(), time);
    }
    if start_time > enddatetime {
        start_time = enddatetime;
    }
    // Query and print nearest available rooms
    let keys = get_rooms(
        reload,
        &config.room.to_string(),
        10,
        start_time,
        enddatetime,
    )
    .await
    .expect("Fehler bei get_rooms");
    println!("neares rooms from {} are: ", config.room.to_string());
    for (roomname, distance) in keys {
        println!("{} (distance: {})", roomname, distance);
    }
}
