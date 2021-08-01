mod timer;
use timer::Timer;

use std::fmt::Error;
use std::io;
use std::io::Write; // Import flush
use std::{thread::sleep, time::Duration, time::Instant};
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;


// Get input as a string
fn input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?.to_string();
    Ok(input)
}

fn confirm() -> bool {
    print!("Continue? (Y): "); io::stdout().flush().unwrap();
    let input: String = input().unwrap();
    if input == "\n" || input == "\r\n" || input.trim().to_lowercase() == "y" {
        true
    } else {
        false
    }
}

// To-do: Change regexes to use crate lazy static macro

// Input must be input in the format #h#, #h#m, #h, #m, or #
// The total time in seconds must fit in a u16
fn validate_time_format(s: &str) -> bool {
    // Check format
    let re = Regex::new(r"^\d+$|^\d+h$|^\d+h\d+$|^\d+m$|^\d+h\d+m$").unwrap();
    if !re.is_match(s) {
        return false
    }
    
    let mut hours: u16 = 0;
    match get_hours(s) {
        Ok(n) => { hours = n; },
        Err(_) => {}
    }
    let mut minutes: u16 = 0;
    match get_minutes(s) {
        Ok(n) => { minutes = n; },
        Err(_) => {}
    }
    let mut hours_as_seconds: u16 = 0;
    match hours.checked_mul(60*60) {
        Some(n) => { hours_as_seconds = n; }
        None => { return false }
    }
    let mut minutes_as_seconds: u16 = 0;
    match minutes.checked_mul(60) {
        Some(n) => { minutes_as_seconds = n; }
        None => { return false }
    }
    match hours_as_seconds.checked_add(minutes_as_seconds) {
        Some(n) => {
            if n > 0 {
                return true
            } else {
                return false
            }
        }
        None => { return false }
    }
}

fn get_total_seconds(s: &str) -> u16 {
    let mut hours: u16 = 0;
    match get_hours(s) {
        Ok(n) => { hours = n; },
        Err(_) => {} // No hours were input, leave as 0
    }
    let mut minutes: u16 = 0;
    match get_minutes(s) {
        Ok(n) => { minutes = n; },
        Err(_) => {} // No minutes were input, leave as 0
    }
    let hours_as_seconds: u16 = hours * 60 * 60;
    let minutes_as_seconds: u16 = minutes * 60;
    println!("secs: {}",hours_as_seconds + minutes_as_seconds);
    hours_as_seconds + minutes_as_seconds
}

//// To-do: Add case insensitivity for h and m chars
fn get_hours(s: &str) -> Result<u16, &'static str> {
    let re = Regex::new(r"\d+h").unwrap();
    match re.find(s) {
        Some(s) => {
            let hours_str = s.as_str().replace("h","");
            match hours_str.parse::<u16>() {
                Ok(n) => {
                    Ok(n)
                }
                Err(_) => {
                    Err("Err")
                }
            }
        },
        None => {
            Err("Err")
        }
    }
}
fn get_minutes(s: &str) -> Result<u16, &'static str> {
    let re = Regex::new(r"\d+m|\d+$").unwrap();
    match re.find(s) {
        Some(s) => {
            let hours_str = s.as_str().replace("m","");
            match hours_str.parse::<u16>() {
                Ok(n) => {
                    Ok(n)
                }
                Err(_) => {
                    Err("Err")
                }
            }
        },
        None => {
            Err("Err")
        }
    }
}

fn main() {
    //println!("Enter times for work, break, and long break in minutes, and the number of iterations before \
    // the long break time activates, separated by spaces.\n\ne.g. \"55 5 25 3\" to work for 55 minutes for \
    // 3 cycles with a 5 min break, followed by a cycle with a 25 minute break:\n");
    
    // Set up variables to handle input data
    let (mut work_time, mut break_time, mut extended_time, mut num_iterations): (i32, i32, i32, i32) = (0,0,0,0);

    // filter each input on regex
    // parse hours/mins from inputs
    // change to use u64
    //

    // Loop to collect input until it's valid
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure that the print macro prints

        let input: String = input().unwrap();

        // Parse individual strings from input into a vector
        let list: Vec<u16> = input
            .trim() // Remove any padding spaces
            .split(" ") // Use a space as a delimiter
            .filter(|s| validate_time_format(s))
            .map(|s| get_total_seconds(s))
            .take(4) // Take the first 4 inputs that pass the filters
            .collect();
        
        //// Build the list if it's valid. Otherwise, retry.
        //if list.len() >= 4 {
        //    // Set variables from the input
        //    for (i,key) in list.iter().enumerate() {
        //        let n = key.parse::<i32>().unwrap();
        //        match i {
        //            0 => work_time = n,
        //            1 => break_time = n,
        //            2 => extended_time = n,
        //            3 => num_iterations = n,
        //            _ => {}
        //        }
        //    }
        //    println!("Work: {}, Break: {}, Long Break: {}, Iterations: {}", work_time, break_time, extended_time, num_iterations);
        //    if confirm() {
        //        break;
        //    }
        //}
    }

    // Clear to the bottom of the command line
    for _i in 1..100 {
        println!("\n");
    }

    // Loop through cycles
    //loop{
    //    for iter_num in 0..=(num_iterations) { // Inclusive
    //        start_timer(work_time, "work");
    //        if iter_num < num_iterations {    
    //            start_timer(break_time, "break");
    //        } else {
    //            start_timer(extended_time, "long break");
    //        }
    //    }
    //}

//    fn start_timer(mins: i32, iteration_type: &str) {
        //let timer = Timer {
        //    start_time: SystemTime::now(),
        //    total_seconds: mins * 60,
        //    current_second: 0
        //};
        //display_time(&mins, &timer.current_second, iteration_type); // Display time initially
        //loop {
        //    match start_time.elapsed() {
        //        Ok(elapsed) => {
        //            let time = elapsed.as_secs() as i32; // Get cumulative time since timer start
        //            if time > elapsed_seconds { // If the second has incremented
        //                elapsed_seconds += time - elapsed_seconds; // Add the difference in seconds to the second counter
        //                // Get minutes value to display. Total minutes minus minutes elapsed, then minus one to account for the fractional minute held in the seconds variable.
        //                let mut display_minutes: i32 = mins - elapsed_seconds / 60 - 1;
        //                // Get seconds value to display. Take the total seconds minus elapsed seconds, then the remainder of that value divided by 60 seconds.
        //                let display_seconds: i32 = (total_seconds - elapsed_seconds) % 60;
        //                if display_seconds == 0 {
        //                    display_minutes += 1; // For a non-fractional minute, don't floor the minute value
        //                }
        //                if elapsed_seconds == total_seconds {
        //                    play_sound(if iteration_type=="work" {
        //                        "BreakSound.wav"
        //                    } else { 
        //                        "WorkSound.wav" 
        //                    });
        //                    break; // Break from the loop if the last second has been reached
        //                }
        //                display_time(&display_minutes, &display_seconds, iteration_type);
        //            }
        //        }
        //        Err(e) => {
        //            println!("Error: {:?}", e);
        //        }
        //    }
        //    sleep(Duration::from_millis(100)); // Polling rate
        //}
    //}
    
    // Display formatted time 
    fn display_time(&mins: &i32, &secs: &i32, iteration_type: &str) {
        //Print the time to wait in MM:SS format
        let display_mins: String = if mins < 10 { format!("0{}",mins.to_string()) } else { format!("{}",mins.to_string()) };
        let display_secs: String = if secs < 10 { format! ("0{}",secs.to_string()) } else { format! ("{}",secs.to_string()) };
        println!("{}:{} {}",display_mins,display_secs, iteration_type);
    }

    fn play_sound(file_name: &str) {
        let device = rodio::default_output_device().unwrap();
        let file = File::open(file_name).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
    }
}
