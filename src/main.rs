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
    print!("Continue? (Y): ");
    io::stdout().flush().unwrap();

    let input: String = input().unwrap();
    if input == "\n" || input == "\r\n" || input.trim().to_lowercase() == "y" {
        true
    } else {
        false
    }
}

// To-do: Change regexes to use crate lazy static macro

// Input must be input in the format #h#, #h#m, #h, #m, or #
// The total time in seconds must fit in a u32
fn valid_time_format(s: &str) -> bool {
    // Check format
    let re = Regex::new(r"^\d+$|^\d+h$|^\d+h\d+$|^\d+m$|^\d+h\d+m$").unwrap();
    if !re.is_match(s) {
        return false
    }
    
    let mut hours: u32 = 0;
    match get_hours(s) {
        Ok(n) => { hours = n; },
        Err(_) => {}
    }
    let mut minutes: u32 = 0;
    match get_minutes(s) {
        Ok(n) => { minutes = n; },
        Err(_) => {}
    }
    let mut hours_as_seconds: u32 = 0;
    match hours.checked_mul(60*60) {
        Some(n) => { hours_as_seconds = n; }
        None => { return false }
    }
    let mut minutes_as_seconds: u32 = 0;
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

// Get total seconds from hours and minutes, which has already been checked for an overflow.
fn get_total_seconds(s: &str) -> u32 {
    let mut hours: u32 = 0;
    match get_hours(s) {
        Ok(n) => { hours = n; },
        Err(_) => {} // No hours were input, leave as 0
    }
    let mut minutes: u32 = 0;
    match get_minutes(s) {
        Ok(n) => { minutes = n; },
        Err(_) => {} // No minutes were input, leave as 0
    }
    let hours_as_seconds: u32 = hours * 60 * 60;
    let minutes_as_seconds: u32 = minutes * 60;
    //println!("secs: {}",hours_as_seconds + minutes_as_seconds);
    hours_as_seconds + minutes_as_seconds
}

//// To-do: Add case insensitivity for h and m chars
fn get_hours(s: &str) -> Result<u32, &'static str> {
    let re = Regex::new(r"\d+h").unwrap();
    match re.find(s) {
        Some(s) => {
            let hours_str = s.as_str().replace("h","");
            match hours_str.parse::<u32>() {
                Ok(n) => { Ok(n) }
                Err(_) => { Err("Err") }
            }
        },
        None => { Err("Err") }
    }
}
fn get_minutes(s: &str) -> Result<u32, &'static str> {
    let re = Regex::new(r"\d+m|\d+$").unwrap();
    match re.find(s) {
        Some(s) => {
            let hours_str = s.as_str().replace("m","");
            match hours_str.parse::<u32>() {
                Ok(n) => { Ok(n) }
                Err(_) => { Err("Err") }
            }
        },
        None => { Err("Err") }
    }
}

// Whether the string can parse as an i32
fn is_u32(string: &str) -> bool {
    let result = string.parse::<u32>();
    match result {
        Ok(_)=> { true },
        Err(_)=> { false }
    }
}

fn main() {
    //println!("Enter times for work, break, and long break in minutes, and the number of iterations before \
    // the long break time activates, separated by spaces.\n\ne.g. \"55 5 25 3\" to work for 55 minutes for \
    // 3 cycles with a 5 min break, followed by a cycle with a 25 minute break:\n");
    
    // Set up variables to handle input data
    let (mut work_time, mut break_time, mut extended_time, mut num_iterations): (u32, u32, u32, u32) = (0,0,0,0);

    // Loop to collect input until it's valid
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure that the print macro prints

        let input: String = input().unwrap();
        let tokens: Vec<&str> = input.trim().split(" ").collect();
        match tokens.get(3) {
            Some(_) => {}
            None => {
                println!("\nInvalid number of inputs.");
                continue;
            }
        }

        // Validate and assign time periods
        let mut is_valid: bool = true;
        for (i, time_period) in vec![tokens[0], tokens[1], tokens[3]].iter().enumerate() {
            if valid_time_format(time_period) {
                match i {
                    0 => work_time = get_total_seconds(*time_period),
                    1 => break_time = get_total_seconds(*time_period),
                    2 => extended_time = get_total_seconds(*time_period),
                    _ => {}
                }
            } else {
                println!("\n{} is an invalid time period.", time_period);
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }

        // Validate and assign iterations
        match tokens[2].parse::<u32>() {
            Ok(n) => { num_iterations = n; }
            Err(_) => {
                println!("\n{} is an invalid number of iterations.", tokens[2]);
                continue;
            }
        }

        // Confirm input
        println!("Work: {}, Break: {}, Long Break: {}, Iterations: {}", work_time, break_time, extended_time, num_iterations);
        if confirm() {
            break;
        }
    }

    // Clear to the bottom of the command line
    for _i in 1..100 {
        println!("\n");
    }

    // Loop through cycles
    loop{
       for iter_num in 0..=(num_iterations) { // Inclusive
           start_timer(work_time, "work");
           if iter_num < num_iterations {    
               start_timer(break_time, "break");
           } else {
               start_timer(extended_time, "long break");
           }
       }
    }

    fn start_timer(total_seconds: u32, iteration_type: &str) {
        let mut timer = Timer::new(Instant::now(), total_seconds);
        println!("{} {}", timer.value(), iteration_type);
        while !timer.is_done() {
            if timer.is_next_second() {
                println!("{} {}", timer.value(), iteration_type);
            }
            sleep(Duration::from_millis(100)); // Polling rate
        }
        play_sound(if iteration_type=="work" {
            "BreakSound.wav"
        } else { 
            "WorkSound.wav" 
        });
        //display_time(&mins, &timer.current_second, iteration_type); // Display time initially
        // loop {
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
        // }
    }

    fn play_sound(file_name: &str) {
        let device = rodio::default_output_device().unwrap();
        let file = File::open(file_name).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
    }
}
