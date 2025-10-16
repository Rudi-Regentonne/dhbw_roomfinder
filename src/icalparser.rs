use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, read_to_string, File},
    io::Write,
    path::Path,
};

use icalendar::{Calendar, CalendarComponent, Component, Event, EventLike};

pub fn parse_calendar(filename: &str) -> HashMap<String, Vec<Event>> {
    let filename = filename;

    let contents = read_to_string(Path::new(&filename)).unwrap();

    let parsed_calendar: Calendar = contents.parse().unwrap();

    let mut events = HashMap::new();
    let coursename = filename.replace("courses/KA-", "").replace(".ics", "");
    for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            let mut new_event = event.clone();
            new_event.append_property(("X-KURS", coursename.as_str()));
            new_event.append_property(("RESOURCES", coursename.as_str()));
            let summary = new_event.get_summary().unwrap_or("").to_string();
            new_event.summary(&format!("{} ({})", summary, coursename));
            let start = new_event
                .get_start()
                .map(|dt| format!("{:?}", dt))
                .unwrap_or("".to_string());

            let key = format!("{}|{}", summary, start);
            events.entry(key).or_insert_with(Vec::new).push(new_event);
        }
    }
    return events;
}

pub fn parse_all_calendars() -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir("courses").unwrap();
    let mut events: HashMap<String, Vec<Event>> = Default::default();
    for path in paths {
        let coursename = path.unwrap().path().display().to_string();
        println!("File: {}", coursename);

        let new_events = parse_calendar(&coursename);
        join_hashmap(&mut events, new_events);
    }

    let mut locations: HashMap<String, Vec<Event>> = HashMap::new();
    let re = Regex::new(r"([a-gA-G]\d{3})").unwrap();
    for (_key, event_vec) in events {
        for event in event_vec {
            //println!("{}", key);
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
    }
    fs::create_dir_all("rooms")?;
    for (location, events) in locations {
        let mut calendar = Calendar::new();
        calendar.name(&location);
        for event in events {
            calendar.push(event);
        }
        let mut file = File::create(format!("rooms/{}.ics", location.replace("/", "_")))?;
        file.write_all(calendar.to_string().as_bytes())?;
    }
    Ok(())
}

fn join_hashmap(hash1: &mut HashMap<String, Vec<Event>>, hash2: HashMap<String, Vec<Event>>) {
    for (key, mut events_b) in hash2 {
        hash1
            .entry(key)
            .or_insert_with(Vec::new)
            .append(&mut events_b);
    }
}
