use chrono::{Duration, Utc};
mod config;
use clap::Parser;
use config::Config;
use dhbw_roomfinder::get_rooms;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'r', long = "room")]
    room: Option<String>,
    #[arg(short = 'f', long = "refetch")]
    refetch: bool,
}
mod room;
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut config = Config::get_config(args.room).expect("failed to get config");

    let reload = config.last_updated < Utc::now() - Duration::days(1) || args.refetch;
    if reload {
        config.last_updated = Utc::now();
        let _ = config.save();
    }
    let keys = get_rooms(reload, &config.room.to_string(), 10)
        .await
        .expect("Fehler bei get_rooms");
    println!("neares rooms from {} are: ", config.room.to_string());
    for (roomname, distance) in keys {
        println!("{} (distance: {})", roomname, distance);
    }
}
