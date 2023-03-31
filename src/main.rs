mod timer;
use timer::Timer;

use tokio;
use std::io;
use std::io::Write; // Import flush
use std::{thread::sleep, time::Duration, time::Instant};
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use rodio::{source::Source, Decoder, OutputStream, Sink};

/*
Enter times for work, break, and long break in minutes, and the number of iterations before the long break time activates, separated by spaces.
e.g. "1h45 15 30 3" to work for 1 hour 45 minutes with a 15 min break, and a 30 minutes break after 3 cycles.

Times can be input as #h#, #h#m, #h, #m, or #. Numbers with no label are taken as minutes.
    For 1 hour and 45 minutes, acceptable inputs are 1h45, and 1h45m.
    For 1 hour, acceptable input is 1h.
    For 30 minutes, acceptable inputs are 30m, and 30.
*/

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

//// To-do: Change regexes to use crate lazy static macro

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
    let hours_as_seconds: u32;
    match hours.checked_mul(60*60) {
        Some(n) => { hours_as_seconds = n; }
        None => { return false }
    }
    let minutes_as_seconds: u32;
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

// Get total seconds from hours and minutes, which have already been checked for an overflow.
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

#[tokio::main]
async fn main() {
    println!("Enter times for work, break, and long break in minutes, and the number of iterations before \
    the long break time activates, separated by spaces.\n\ne.g. \"1h45 15 30 3\" to work for 1 hour 45 minutes \
    with a 15 min break, and a 30 minutes break after 3 cycles:\n");
    
    // Define variables to handle input data
    let (mut work_time, mut break_time, mut extended_time): (u32, u32, u32) = (0,0,0);
    let mut num_iterations: u32;

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
                    0 => work_time = get_total_seconds(time_period),
                    1 => break_time = get_total_seconds(time_period),
                    2 => extended_time = get_total_seconds(time_period),
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
           start_timer(work_time, "work").await;
           if iter_num < num_iterations {    
               start_timer(break_time, "break").await;
           } else {
               start_timer(extended_time, "long break").await;
           }
       }
    }
}

async fn start_timer(total_seconds: u32, iteration_type: &'static str) {
    let mut timer = Timer::new(Instant::now(), total_seconds);

    let join_handle = tokio::spawn(async move {
        play_sound(if iteration_type == "work" {
            "BreakSound.wav"
        } else { 
            "WorkSound.wav" 
        });
    });

    timer.wait_and_print(iteration_type).await;

    // Cycle time must be longer than the sound file to prevent a delay from awaiting..
    // This should be fine since the smallest cycle input possible is 1 minute.
    join_handle.await;
}

fn play_sound(file_name: &str) {
    // Load the sound file
    let file = File::open(file_name).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Create an output stream and sink
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    // Add the sound to the sink
    sink.append(source);
    // Wait for the duration of the sound
    sink.sleep_until_end();
}
