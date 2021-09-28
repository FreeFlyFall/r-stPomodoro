use std::time::Instant;
use math::round;

pub struct Timer {
    pub start_time: Instant,
    pub total_seconds: u32,
    pub current_second: u32,
}

impl Timer {
    pub fn new(start_time: Instant, total_seconds: u32) -> Timer {
        Timer {
            start_time: start_time,
            total_seconds: total_seconds,
            current_second: 0,
        }
    }

    pub fn is_next_second(&mut self) -> bool {
        let elapsed: u32 = self.start_time.elapsed().as_secs() as u32;
        let is_next: bool = elapsed > self.current_second;
        self.current_second += elapsed - self.current_second;
        is_next
    }

    pub fn value(&self) -> String {
        let elapsed: u32 = self.start_time.elapsed().as_secs() as u32;

        let seconds: u32 = (self.total_seconds - elapsed) % 60;
        let minutes: u32 = round::floor((self.total_seconds as f64 / 60 as f64 - elapsed as f64 / 60 as f64) % 60 as f64, 0) as u32;
        let mut hours: u32 = round::floor(self.total_seconds as f64 / 3600 as f64 - elapsed  as f64 / 3600 as f64, 0) as u32;
        if seconds == 0 && minutes == 0 && elapsed != 0 {
            hours += 1
        }

        let display_hours: String = Timer::format_time(&hours);
        let display_minutes: String = Timer::format_time(&minutes);
        let display_seconds: String = Timer::format_time(&seconds);

        let display_time: String = "".to_owned() +
        &display_hours +
        if display_hours.is_empty() {""} else {":"} + 
        &display_minutes +
        if display_minutes.is_empty() {""} else {":"} +
        &display_seconds;
        
        display_time
    }

    pub fn is_done(&self) -> bool {
        self.start_time.elapsed().as_secs() as u32 >= self.total_seconds
    }

    fn format_time(&time: &u32) -> String {
        if time < 10 {
            format!("0{}",time.to_string())
        } else {
            format!("{}",time.to_string())
        }
    }
}