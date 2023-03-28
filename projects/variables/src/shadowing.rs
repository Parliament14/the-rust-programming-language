fn main() { 

    // First, delcare an immutable variable named "x" and assign it a value of 5 
    let x = 5; 

    // Using "shadowing", we can change both the value and the type of the error using an expression like so: 
    let x = x + 1; 

    // Then, we can define a new scope using the curly brackets 
    { 
        // Once again, we "shadow" x and change it's value to the result of the following expression 
        let x = x * 2; 
        println!("The value of x in the inner scope is {x}"); 

    }

    println!("the value of x is {x}")
}