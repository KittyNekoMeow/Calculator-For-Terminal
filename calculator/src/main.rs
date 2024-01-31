use std::io;

fn calculate(x: f64, a:String, y: f64)-> f64 {

    if a == "plus" {
        x + y
    }else if a == "minus"{
        x - y
    }else if a == "times"{
        x * y
    }else if a == "divide" {
        x / y
        
    }else {
        x
    }

}



fn main() {

   
   println!("Type plus, minus, times, or divide inbetween numbers");
   
    let mut input1 = String::new();

    let mut input2 = String::new();

    let mut input3 = String::new();

    io::stdin().read_line(&mut input1).unwrap();

    io::stdin().read_line(&mut input2).unwrap();

    io::stdin().read_line(&mut input3).unwrap();

    let num1: f64 = input1.trim().parse().unwrap();

    let text1: String = input2.trim().parse().unwrap();
    
    let num2: f64 = input3.trim().parse().unwrap();

    let c:f64 = calculate(num1, text1, num2);

    println!("The answer is: {}", c);

}
