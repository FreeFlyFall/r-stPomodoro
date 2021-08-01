use std::time::Instant; 

pub struct Timer {
    pub start_time: Instant,
    pub total_seconds: u64,
    pub current_second: u64,
}

impl Timer {
    pub fn new(start_time: Instant, total_seconds: u64) -> Timer {
        Timer {
            start_time: start_time,
            total_seconds: total_seconds,
            current_second: 0,
        }
    }
    pub fn is_next_second(&self) -> bool {
        //let elapsed: i32 = self.start_time.elapsed()?.as_secs();
        //let is_next: bool = elapsed > self.current_second;
        //self.current_second += elapsed - self.current_second;
        //is_next
        true
    }
    pub fn value() -> String {
        "String".to_string()
    }
    pub fn is_done() -> bool {
        true
    }
}