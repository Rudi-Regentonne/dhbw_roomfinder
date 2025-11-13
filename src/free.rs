use chrono::{Local, NaiveDateTime};
use icalendar::{Calendar, CalendarComponent, CalendarDateTime, Component, DatePerhapsTime};
use rayon::prelude::*;

use std::fs;

/// Prints the names of all events from a room's calendar that occur today.
/// - `path`: The room identifier (filename in rooms/)
#[allow(dead_code)]
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

/// Checks if a room is free for the given datetime.
/// Returns true if no event occupies the room at that time,
/// or if only all-day events are present on the same date.
/// - `path`: The room identifier (filename in rooms/)
/// - `datetime`: The local date and time to check
pub fn is_free(path: &str, datetime: NaiveDateTime) -> bool {
    let content = fs::read_to_string(format!("rooms/{}.ics", path)).unwrap();
    let calendar: Calendar = content.parse().unwrap();

    let belegt = calendar.components.par_iter().any(|component| {
        if let CalendarComponent::Event(event) = component {
            if let (Some(dtstart), Some(dtend)) = (event.get_start(), event.get_end()) {
                let start = match dtstart {
                    DatePerhapsTime::DateTime(dt) => match dt {
                        CalendarDateTime::Utc(dt_utc) => dt_utc.naive_local(),
                        CalendarDateTime::Floating(naive_dt) => naive_dt,
                        CalendarDateTime::WithTimezone { date_time, .. } => date_time,
                    },
                    DatePerhapsTime::Date(date) => return date == datetime.date(),
                };

                let end = match dtend {
                    DatePerhapsTime::DateTime(dt) => match dt {
                        CalendarDateTime::Utc(dt_utc) => dt_utc.naive_local(),
                        CalendarDateTime::Floating(naive_dt) => naive_dt,
                        CalendarDateTime::WithTimezone { date_time, .. } => date_time,
                    },
                    DatePerhapsTime::Date(date) => {
                        return date == datetime.date();
                    }
                };

                return (start >= datetime || end >= datetime) && end.date() == datetime.date();
            }
        }
        false
    });

    return !belegt;
}
