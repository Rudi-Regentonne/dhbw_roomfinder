use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("data.json").exists() {
        let body = get_page().await;
        match body {
            Ok(text) => {
                println!("body = {text:?}");
                write_file(text).expect("Error writing file");
            }
            Err(e) => println!("Fehler: {e}"),
        }
    }
    let json_str = fs::read_to_string("data.json").expect("Should have been able to read the file");
    let json: Value = serde_json::from_str(&json_str)?;

    for bereich in json.as_object().unwrap().values() {
        for fach in bereich.as_object().unwrap().values() {
            if let Some(kurse) = fach.as_array() {
                for kurs in kurse {
                    if let Some(name) = kurs.get("name").and_then(|n| n.as_str()) {
                        println!("{}", name);
                        if !Path::new(&(format!("courses/{}.ics", name))).exists() {
                            let _ = download(name).await;
                        } else {
                            println!("{name}")
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

async fn get_page() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://api.dhbw.app/courses/MA/mapped/extended")
        .await?
        .text()
        .await?;
    Ok(body)
}

fn write_file(contents: String) -> io::Result<()> {
    let mut file = File::create("data.json")?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
async fn download(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading: {name}.ics");
    let url = format!("https://dhbw.app/ical/{}", name);
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let mut out = File::create(format!("courses/{}.ics", name))?;
    out.write_all(&bytes)?;
    Ok(())
}
