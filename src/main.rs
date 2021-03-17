use std::io;
use std::io::Write; // Import flush
use std::{thread::sleep, time::Duration, time::SystemTime};
use std::fs::File;
use std::io::BufReader;
use rodio::Source;

// Get input as a string
fn input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?.to_string();
    Ok(input)
}

// Whether the string can parse as an i32
fn is_i32(string: &str) -> bool {
    let result = string.parse::<i32>();
    match result {
        Ok(_)=> {
            true
        },
        Err(_)=> {
            println!("{} is not an integer\n", string);
            false
        }
    }
}

// Whether the minutes entered are under a workday. Prevents overflowing the i32 which holds total seconds.
fn is_under_day(string: &str) -> bool {
    let result = string.parse::<i32>();
    match result {
        Ok(number)=> {
            if number < 24 * 60 {
                true
            } else {
                println!("{} minutes is more than 24 hours\n", number);    
                false 
            }
        },
        Err(_)=> {
            false
        }  
    }
}

// Whether the string, parsed as an i32, is > 0
fn is_positive(string: &str) -> bool {
    if string.parse::<i32>().unwrap() > 0 {
        true
    } else {
        println!("{} is not a positive integer\n", string);
        false
    }
}

fn main() {
    println!("Enter times for work, break, and long break in minutes, and the number of iterations before \
     the long break time activates, separated by spaces.\n\ne.g. \"55 5 25 3\" to work for 55 minutes for \
     3 cycles with a 5 min break, followed by a cycle with a 25 minute break:\n");
    
    // Set up variables to handle input data
    let (mut work_time, mut break_time, mut extended_time, mut num_iterations): (i32, i32, i32, i32) = (0,0,0,0);

    // Loop to collect input until it's valid
    loop {
        let input = input().unwrap();

        // Parse individual strings from input into a vector
        let list: Vec<&str> = input
            .trim() // Remove any padding spaces
            .split(" ") // Use a space as a delimiter
            .filter(|s| is_i32(s)) // Take only integers
            .filter(|s| is_positive(s)) // Take only positive numbers
            .filter(|s| is_under_day(s)) // Take only periods under a day
            .take(4) // Take the first 4 inputs that pass the filters
            .collect();
        
        // Build the list if it's valid. Otherwise, retry.
        if list.len() >= 4 {
            // Set variables from the input
            for (i,key) in list.iter().enumerate() {
                let n = key.parse::<i32>().unwrap();
                match i {
                    0 => work_time = n,
                    1 => break_time = n,
                    2 => extended_time = n,
                    3 => num_iterations = n,
                    _ => {}
                }
            }
            break
        } else {
            print!("Retry: "); io::stdout().flush().unwrap(); // Ensure that the print macro prints
        }
    }

    // Clear to the bottom of the command line
    for _i in 1..100 {
        println!("\n");
    }

    // Loop through cycles
    loop{
        for iter_num in 0..(num_iterations+1) {
            start_timer(work_time, "work");
            play_sound("BreakSound.wav");
            if iter_num < num_iterations {    
                start_timer(break_time, "break");
                play_sound("WorkSound.wav");
            } else {
                start_timer(extended_time, "long break");
                play_sound("WorkSound.wav");
            }
        }
    }

    fn start_timer(mins: i32, iteration_type: &str) {
        let now = SystemTime::now();
        let total_seconds: i32 = mins * 60; // Total time as seconds
        let mut elapsed_seconds: i32 = 0; // Variable to keep track of elapsed seconds
        display_time(&mins, &elapsed_seconds, iteration_type); // Display time initially
        loop {
            match now.elapsed() {
                Ok(elapsed) => {
                    let time = elapsed.as_secs() as i32; // Get cumulative time since timer start
                    if time > elapsed_seconds { // If the second has incremented
                        elapsed_seconds += time - elapsed_seconds; // Add the difference in seconds to the second counter
                        // Get minutes value to display. Total minutes minus minutes elapsed, then minus one to account for the fractional minute held in the seconds variable.
                        let mut display_minutes = mins - elapsed_seconds / 60 - 1;
                        // Get seconds value to display. Take the total seconds minus elapsed seconds, then the remainder of that value divided by 60 seconds.
                        let display_seconds = (total_seconds - elapsed_seconds) % 60;
                        if display_seconds == 0 {
                            display_minutes += 1; // For a non-fractional minute, don't floor the minute value
                        }
                        if elapsed_seconds == total_seconds {
                            break; // Break from the loop if the last second has been reached
                        }
                        display_time(&display_minutes, &display_seconds, iteration_type);
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            sleep(Duration::from_millis(100)); // Polling rate
        }
    }
    
    // Display formatted time and wait one second
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
