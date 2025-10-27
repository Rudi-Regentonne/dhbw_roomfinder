use chrono::{Duration, Local};
use icalendar::{Calendar, CalendarComponent, CalendarDateTime, Component, DatePerhapsTime};
use rayon::prelude::*;

use std::fs;

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

// returns free if the room is free for today
pub fn is_free(path: &str) -> bool {
    let content = fs::read_to_string(format!("rooms/{}.ics", path)).unwrap();
    let calendar: Calendar = content.parse().unwrap();
    let today = Local::now().date_naive() + Duration::hours(0);
    let now = Local::now().naive_local() + Duration::hours(0);

    let belegt = calendar.components.par_iter().any(|component| {
        if let CalendarComponent::Event(event) = component {
            if let (Some(dtstart), Some(dtend)) = (event.get_start(), event.get_end()) {
                let start = match dtstart {
                    DatePerhapsTime::DateTime(dt) => match dt {
                        CalendarDateTime::Utc(dt_utc) => dt_utc.naive_local(),
                        CalendarDateTime::Floating(naive_dt) => naive_dt,
                        CalendarDateTime::WithTimezone { date_time, .. } => date_time,
                    },
                    DatePerhapsTime::Date(date) => return date == today,
                };

                let end = match dtend {
                    DatePerhapsTime::DateTime(dt) => match dt {
                        CalendarDateTime::Utc(dt_utc) => dt_utc.naive_local(),
                        CalendarDateTime::Floating(naive_dt) => naive_dt,
                        CalendarDateTime::WithTimezone { date_time, .. } => date_time,
                    },
                    DatePerhapsTime::Date(date) => {
                        return date == today;
                    }
                };

                return (start >= now || end >= now) && end.date() == today;
            }
        }
        false
    });

    return !belegt;
}
