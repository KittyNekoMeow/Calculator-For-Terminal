use std::io;
use std::fmt::Error;

// Do the maths
fn calculate(in1: f64, symbol: Option<char>, in2: f64) -> f64 {
   
    //  Checks for the correct symbol
    match symbol {
        Some('+') => in1+in2,
        Some('-') => in1-in2,
        Some('*') => in1*in2,
        Some('/') => in1/in2,
       
       // Give arbitrary number so loop breaks
        _ => 546343329.654352103
    }
}

// Checks if input is a float
fn check(number: &str) -> Result<f64, Error> {
    match number.parse::<f64>() {
        Ok(i) => Ok(i),
        Err(_) => Err(Error)
    }
}

fn main() {
   
    loop {
        
    // Print instructions
    println!("Type +, -, *, or / inbetween numbers");

    let mut input1 = String::new();

    let mut input2 = String::new();

    let mut input3 = String::new();

    // Takes input
    io::stdin().read_line(&mut input1).unwrap();

    io::stdin().read_line(&mut input2).unwrap();

    io::stdin().read_line(&mut input3).unwrap();

    // Convert input
    let num1 = input1.trim();

    // Chatches unwrap err of to many chars
    let text: String = input2.trim().parse().unwrap();

    let num2 = input3.trim();
    
    let text_char = text.chars().next();

    // If variable isn't a float break the loop
    let num1 = match check(num1) {
        Ok(i) => i,
        Err(_) => {println!("Did not input number"); return}
    };

    // Same as first
    let num2 = match check(num2) {
        Ok(i) => i,
        Err(_) => {println!("Did not input number"); return}
    };

    // Calls for calculation
    let answer = calculate(num1, text_char, num2);

    // Checks if you didn't input a modifier
    if answer == 546343329.654352103 {
        println!("Use the correct symbol next time");
        
        // Cancels loop if didn't
        return
    }

    // Prints answer
    println!("The answer is: {}", answer);

    println!("To exit program enter q, to continue press any other key");

    // Takes new input
    let mut input4 = String::new();

    io::stdin().read_line(&mut input4).unwrap();

    let exit_code: String = input4.trim().parse().unwrap();
   
    // If input is "q" break the loop
    if exit_code == "q" {
        return
    }

 }

}
