use std::io;

// Do the maths
fn calculate(in1: f64, symbol: char, in2: f64) -> f64 {
   
    //  Checks for the correct symbol
    match symbol {
        '+' => in1+in2,
        '-' => in1-in2,
        '*' => in1*in2,
        '/' => in1/in2,
       
       // Give arbitrary number so loop breaks
        _ => 546343329.654352103,
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
    let num1 = input1.trim().parse().unwrap();

    let text: char = input2.trim().parse().unwrap();

    let num2 = input3.trim().parse().unwrap();

    // Calls for calculation
    let answer = calculate(num1, text, num2);

    // Checks if you didn't input a modifier
    if answer == 546343329.654352103 {
        println!("Use the correct symbol next time");
        
        // Cancels loop if didn't
        break
    }

    // Prints answer
    println!("The answer is: {}", answer);
    
    }

}
