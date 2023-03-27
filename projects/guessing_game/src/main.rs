use std::io; 

fn main(){
    println!("Guess the number!"); 
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
    println!("You guessed: {guess}"); 
}