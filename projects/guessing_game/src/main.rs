use std::io; 
use std::cmp::Ordering; 
use rand::Rng;

fn main(){
    println!("Guess the number!"); 

    // Using the "rand::threaded_rng" function gives us a random number generator
    // Using the "gen_range" method gives us a random number which we assign to the 
    // immutable variable named "secret_number" 
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:"); 
    
        // Declare new instance of mutable string, called 'guess' 
        let mut guess = String::new(); 

        
        // Call the "stdin" function from the "io" module from the "std" library 
        io::stdin()

            // Use the "read_line" method on the standard input 
            // handle to get input from the user 
            .read_line(&mut guess)

            // Use the "expect" method on the standard input 
            // handle to display "Failed to read line" in the event 
            // the "read_line" method returns a "Result" instance with 
            // and "Err" variant. 

            // The above is jargon for "Rust makes you do exception handling" 
            // with the "expect" method each time you call a function 
            .expect("Failed to read line"); 

        
        // Now, we are going to change the "guess" variables type
        // to an unsigned 32 bit integer. The ":" annotates the variables type 
        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num, 
            Err(_) => continue,  
        };
        println!("You guessed: {guess}"); 

        match guess.cmp(&secret_number) { 
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break; 
            }
        }
    }
}