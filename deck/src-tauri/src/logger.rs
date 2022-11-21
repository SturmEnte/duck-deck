use std::fs::File;
use std::io::Write;
use std::path::Path;

use chrono;

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new(path: &str) -> Logger {
        let path = Path::new(path);
        let file: File;

        if !path.exists() {
            file = File::create(path).unwrap();
        } else {
            file = File::open(path).unwrap()
        }

        Logger { file: file }
    }

    pub fn log(&mut self, string: &str) {
        let mut s: String = String::new();
        s.push_str(format!("[{}] ", chrono::offset::Local::now()).as_str());
        s.push_str(string);
        s.push_str("\n");
        self.file.write(s.as_bytes()).unwrap();
        println!("[{}] {}", chrono::offset::Local::now(), s);
    }
}