use std::io;
use std::io::Write; // Import flush
use std::{thread, time::Duration};
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

    fn start_timer(mut mins: i32, iteration_type: &str) {
        // Define a variables for seconds
        let mut secs: i32 = 0;
        // For every minute of this cycle
        while mins >= 0 {
            // Decrement a minute and reset the seconds
            if secs < 0 {  
                mins -= 1;
                secs = 59;
            }
            // If the time has expired, don't wait or print the time
            if mins < 0 { break }

            // Display the timer unless it's the last iteration
            if mins > 0 {
                display_time_and_wait(&mins, &secs, iteration_type);
            } else if secs > 0 {
                display_time_and_wait(&mins,&secs, iteration_type);
            } else {
                break;
            }
            // Subract a second before restarting the loop
            secs -= 1; 
        }
    }
    
    // Display formatted time and wait one second
    fn display_time_and_wait(&mins: &i32, &secs: &i32, iteration_type: &str) {
        // Wait for one second in another thread
        let child = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
        });

        // Print the time to wait in MM:SS format
        let display_mins: String = if mins < 10 { format!("0{}",mins.to_string()) } else { format!("{}",mins.to_string()) };
        let display_secs: String = if secs < 10 { format! ("0{}",secs.to_string()) } else { format! ("{}",secs.to_string()) };
        println!("{}:{} {}",display_mins,display_secs, iteration_type);
        
        // Wait for the sleeping thread
        let _res = child.join();
    }

    fn play_sound(file_name: &str) {
        let device = rodio::default_output_device().unwrap();
        let file = File::open(file_name).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
    }
}
