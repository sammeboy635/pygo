use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self { start_time: Some(Instant::now()) }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn end(&self, message: &str) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            let micros = elapsed.as_micros();
			println!("{}: {} us", message, micros);
        } else {
            println!("Timer not started.");
        }
    }
}