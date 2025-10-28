use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use crate::room::RoomId;

const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub room: RoomId,
    pub last_updated: DateTime<Utc>,
}

impl Config {
    pub fn get_config(room: Option<String>) -> io::Result<Self> {
        if Path::new(CONFIG_FILE).exists() {
            let mut file = File::open(CONFIG_FILE)?;
            let mut json = String::new();
            file.read_to_string(&mut json)?;
            let mut config: Config =
                serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            if let Some(roomname) = room {
                config.room = RoomId::from_str(&roomname).expect("failed to parse roomname");
                let _ = config.save();
            }
            Ok(config)
        } else {
            if let Some(roomname) = room {
                let _ = (&Config {
                    room: RoomId {
                        block: 'A',
                        floor: 2,
                        number: 66,
                    },
                    last_updated: Utc::now(),
                })
                    .save();
                Ok(Config {
                    room: RoomId::from_str(&roomname).expect("failed to parse roomname"),
                    last_updated: Utc::now(),
                })
            } else {
                Ok(Config {
                    room: RoomId {
                        block: ('C'),
                        floor: (0),
                        number: (0),
                    },
                    last_updated: Utc::now(),
                })
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let json_string = serde_json::to_string_pretty(&self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut file = File::create(CONFIG_FILE)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }
}
