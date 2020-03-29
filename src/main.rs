use std::io;
use std::io::Write; // Import flush
use std::{thread, time};
use std::fs::File;
use std::io::BufReader;
use rodio::Source;

fn main() {
    println!("Enter times for work, break, and long break in minutes, and the number of iterations before the long break time activates, separated by spaces.\n\ne.g. \"55 5 25 3\" to work for 55 minutes for three cycles with a 5 min break, followed by a cycle with a 25 minute break:\n");

    let (mut work_time, mut break_time, mut extended_time, mut num_iterations): (i32, i32, i32, i32) = (0,0,0,0);

    let mut done: bool = true;
    loop {
        // Tell the user to retry if there was a failure
        if done == false {
            print!("Retry: ");
            // Ensure that the print macro prints
            io::stdout().flush().unwrap();  
        }

        // Get input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_b) => {}
            Err(error) => println!("Error: {}", error),
        }

        // Parse individual strings from input into a vector
        let tokens:Vec<&str>= input.trim().split(" ").collect();

        // Check that there are 4 inputs
        match tokens.get(3) {
            Some(_n) => {}
            None => {
                println!("\nInvalid number of inputs.");
                done = false;
                continue;
            }
        }
        // Check that the inputs can parse as integers
        for (i,token) in tokens.iter().enumerate() {
            match token.parse::<i32>(){
                Ok(n) => {
                    done = true;
                    if i==0 { work_time = n; }
                    else if i==1 { break_time = n; }
                    else if i==2 { extended_time = n; }
                    else if i==3 { num_iterations = n; }
                }
                Err(_e) => {
                    println!("\n{} is not an integer.",token);
                    done = false;
                    break;
                }
            }
        }
        if done == false { continue; }



        println!("\nWork: {} mins\nBreak: {} mins\nExtended: {} mins\nIterations: {}\n",work_time, break_time, extended_time, num_iterations);
        break;
    }

    // Loop through cycles
    loop{
        for iter_num in 0..(num_iterations+1) {
            start_timer(work_time);
            println!("Take a break.");
            play_sound("BreakSound.wav");
            if iter_num < num_iterations {    
                start_timer(break_time);
                println!("Get back to work.");
                play_sound("WorkSound.wav");
            } else {
                start_timer(extended_time);
                println!("Get backto work.");
                play_sound("WorkSound.wav");
            }
        }
    }

    fn start_timer(mut mins: i32){
        // Define a variables for seconds
        let mut secs: i32 = 0;
        // Every minute
        while mins >= 0 {
            // Decrement a minute and reset the seconds
            if secs < 0 {  
                mins -= 1;
                secs = 59
            }
            // If the time is expired, don't wait or print the time
            if mins < 0 { break }

            // Display the timer unless it's the last iteration
            if mins > 0 {
                display_time_and_wait(&mins, &secs);
            } else if secs > 0 {
                display_time_and_wait(&mins,&secs);
            } else {
                break;
            }
            // Subract a second before restarting the loop
            secs -= 1; 
        } 
    }
    
    // Display formatted time and wait one second
    fn display_time_and_wait(&mins: &i32, &secs: &i32){
        // Print the time to wait in MM:SS format
        let display_mins: String = if mins < 10 { format!("0{}",mins.to_string()) } else { format!("{}",mins.to_string()) };
        let display_secs: String = if secs < 10 { format! ("0{}",secs.to_string()) } else { format! ("{}",secs.to_string()) };
        println!("{}:{}",display_mins,display_secs);
        ////Make sleep concurrent
        thread::sleep(time::Duration::from_millis(10));
    }

    fn play_sound(file_name: &str) {
        let device = rodio::default_output_device().unwrap();
        let file = File::open(file_name).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
    }
}
