use std::time::Instant; 

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
        elapsed += 1;
        let mut hours: u32 = (self.total_seconds / 60 / 60 - elapsed / 60 / 60);
        let mut minutes: u32 = ((self.total_seconds / 60 - elapsed / 60) % 60);
        let seconds: u32 = (self.total_seconds - elapsed) % 60;

        println!("times: {}:{}:{}", self.total_seconds, self.total_seconds/60/60, seconds);

        let display_hours: String = Timer::format_time(&hours,false);
        let display_minutes: String = Timer::format_time(&minutes,false);
        let display_seconds: String = Timer::format_time(&seconds,true);
        
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

    fn format_time(&time: &u32, no_blank: bool) -> String {
        if !no_blank && time == 0 {
            "".to_string()
        }
        else
        if time < 10 {
            format!("0{}",time.to_string())
        } else {
            format!("{}",time.to_string())
        }
    }
}