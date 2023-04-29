use std::time::{Instant, Duration};

pub struct Timer {
    start_time: Instant,
    buffer: String,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: Instant::now(),
            buffer: String::new(),
        }
    }

	pub fn new_start(&mut self) {
        self.start_time = Instant::now();
		self.buffer.push_str("New_Start Time ||");
    }
    pub fn elapse(&mut self, message: &str) {
        let elapsed = self.start_time.elapsed();
        self.buffer.push_str(&format!("{}: {} microseconds ||", message, elapsed.as_micros()));
        self.start_time = Instant::now();
    }

	pub fn elapse_nano(&mut self, message: &str) {
        let elapsed = self.start_time.elapsed();
        self.buffer.push_str(&format!("{}: {} nanoseconds ||", message, elapsed.as_nanos()));
        self.start_time = Instant::now();
    }

    pub fn print(&self) {
        println!("{}", self.buffer);
    }
}