use std::io;

fn calculate(x: f64, a:char, y: f64)-> f64 {

    if a == '+' {
        x+y
    }else if a == '-'  {
        x-y
    }else if a == '*' {
        x*y
    }else if a == '/' {
        x/y
    }else {
        return x
    }

}

fn sym(s: &String)-> char {
    
    if s == "+" {
      return '+'  
    }else if s == "-" {
        return '-'
    }else if s == "*" {
        return '*'
    }else if s == "/" {
        return '/'
    }else {
       return 'X'
    }

}

fn main() {

   
   println!("Type +, -, *, or / inbetween numbers");
   
    let mut input1 = String::new();

    let mut input2 = String::new();

    let mut input3 = String::new();

    io::stdin().read_line(&mut input1).unwrap();

    io::stdin().read_line(&mut input2).unwrap();

    let b: String = input2.trim().parse().unwrap();
   
    let modi: char = sym(&b);

    io::stdin().read_line(&mut input3).unwrap();

    let num1: f64 = input1.trim().parse().unwrap();
    
    let num2: f64 = input3.trim().parse().unwrap();

    let c:f64 = calculate(num1, modi, num2);
   
    println!("The answer is: {}", c);

}
