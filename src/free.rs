use chrono::{Duration, Local};
use icalendar::{Calendar, CalendarComponent, Component};
use rayon::prelude::*;
use std::fs;
pub fn todays_events(path: &str) {
    let content = fs::read_to_string(format!("rooms/{}.ics", path)).unwrap();
    let calendar: Calendar = content.parse().unwrap();

    let today = Local::now().date_naive();

    for component in &calendar.components {
        if let CalendarComponent::Event(event) = component {
            if let Some(dtstart) = event.get_start() {
                if dtstart.date_naive() == today {
                    println!("{}", event.get_summary().unwrap_or("").to_string());
                }
            }
        }
    }
}
pub fn is_free(path: &str) -> bool {
    let content = fs::read_to_string(format!("rooms/{}.ics", path)).unwrap();
    let calendar: Calendar = content.parse().unwrap();

    let today = Local::now().date_naive() + Duration::days(1);

    let belegt = calendar.components.par_iter().any(|component| {
        if let CalendarComponent::Event(event) = component {
            if let Some(dtstart) = event.get_start() {
                return dtstart.date_naive() == today;
            }
        }
        false
    });

    return !belegt;
}
