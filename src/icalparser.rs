use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, read_to_string, File},
    io::Write,
    path::Path,
};

use icalendar::{Calendar, CalendarComponent, Component, Event, EventLike};

use crate::loadingbar::Loadingbar;
/// Parses a single calendar file, extracting events and associating them with course names.
/// Updates the events map with merged or new events as needed.
/// - `filename`: Path to the calendar file (.ics)
/// - `events`: Mutable HashMap to collect unique events
pub fn parse_calendar(filename: &str, events: &mut HashMap<(String, String, String), Event>) {
    let contents = read_to_string(Path::new(&filename)).unwrap();
    let re = Regex::new(r"^.*?/.{3}").unwrap();
    let coursename = re.replace(filename, "").replace(".ics", "");
    let parsed_calendar: Calendar = contents.parse().unwrap();
    for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            let key = (
                event.get_summary().unwrap_or("").to_string(),
                event.get_location().unwrap_or("").to_string(),
                event
                    .get_start()
                    .map(|dt| format!("{:?}", dt))
                    .unwrap_or("".to_string()),
            );
            if let Some(entry) = events.get_mut(&key) {
                let mut courses: Vec<String> = entry
                    .property_value("X-KURS")
                    .unwrap_or("")
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                if !courses.contains(&coursename) {
                    courses.push(coursename.clone());
                    entry.append_property(("X-KURS", courses.join(", ").as_str()));
                    entry.append_property(("RESOURCES", courses.join(",").as_str()));
                }
            } else {
                let mut new_event = event.clone();
                new_event.append_property(("X-KURS", coursename.as_str()));
                new_event.append_property(("RESOURCES", coursename.as_str()));
                events.insert(key, new_event);
            }
        }
    }
}
/// Parses all calendar files in the "courses" directory and groups events by rooms.
/// Writes one *.ics file per room to the "rooms" directory.
/// Uses a loading bar to indicate progress for parsing, grouping, and writing.
/// Returns Ok(()) on success or an error if file I/O fails.
pub fn parse_all_calendars() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<_> = fs::read_dir("courses").unwrap().collect();
    let mut events: HashMap<(String, String, String), Event> = HashMap::new();
    let mut bar = Loadingbar::new("Parsing calendars", paths.len());
    for path in paths {
        let coursename = path.unwrap().path().display().to_string();
        parse_calendar(&coursename, &mut events);
        bar.next();
    }
    println!();

    let mut locations: HashMap<String, Vec<Event>> = HashMap::new();
    let re = Regex::new(r"([a-gA-G]\d{3})").unwrap();
    let mut bar = Loadingbar::new("Creating rooms", events.len());
    for (_key, event) in events {
        bar.next();
        if let Some(location) = event.get_location() {
            let rooms = location.split(", ");
            for room in rooms {
                let roomname = if let Some(caps) = re.captures(room) {
                    caps[1].to_string()
                } else {
                    room.to_string()
                };
                if !roomname.is_empty() {
                    locations
                        .entry(roomname.to_owned())
                        .or_insert_with(Vec::new)
                        .push(event.clone());
                }
            }
        }
    }
    println!();
    fs::create_dir_all("rooms")?;
    let mut bar = Loadingbar::new("Writing rooms to file", locations.len());
    for (location, events) in locations {
        let mut calendar = Calendar::new();
        calendar.name(&location);
        for mut event in events {
            let summary = event.get_summary().unwrap_or("");
            let courses = event.property_value("X-KURS").unwrap_or("");

            event.summary(format!("{} ({})", summary, courses).as_str());
            calendar.push(event);
        }
        let mut file = File::create(format!("rooms/{}.ics", location.replace("/", "_")))?;
        file.write_all(calendar.to_string().as_bytes())?;
        bar.next();
    }
    println!();
    Ok(())
}
