use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Instant,
    end_times: HashMap<String, Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: Instant::now(),
            end_times: HashMap::new(),
        }
    }

    pub fn end_us(&mut self, key: &str) {
        self.end_times.insert(key.to_owned(), Instant::now());
    }

    pub fn print(&self, message: &str) {
        let elapsed = self.start_time.elapsed();
        print!("{}: {} microseconds ||", message, elapsed.as_micros());

        for (key, value) in &self.end_times {
            let elapsed = value.duration_since(self.start_time);
            print!(" {}: {} microseconds ||", key, elapsed.as_micros());
        }
		println!("");
    }
}