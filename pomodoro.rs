use std::io;
use std::io::Write; // Import flush
use std::process::Command;

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
        //let fourth = &tokens[4];
        match tokens.get(3) {
            Some(_n) => {}
            None => {
                println!("\nInvalid number of inputs.");
                done = false;
                continue;
            }        // Parse individual strings from input into a vector
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

    /*  loop { 
        let mut child = Command::new("sleep").arg("1").spawn().unwrap();
        let _result = child.wait().unwrap();
    }*/
            // Use Rodio 
}
