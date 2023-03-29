use std::io;

fn main() {
    let principal = get_input("Enter loan amount: ");
    let interest_rate = get_input("Enter annual interest rate (%): ");
    let loan_term = get_input("Enter loan term (in years): ");
    
    let monthly_rate = interest_rate / 1200.0;
    let num_payments = loan_term * 12.0;
    let numerator = monthly_rate * principal * (1.0 + monthly_rate).powf(num_payments);
    let denominator = (1.0 + monthly_rate).powf(num_payments) - 1.0;
    let monthly_payment = numerator / denominator;
    
    println!("Monthly payment: ${:.2}", monthly_payment);
}

fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}