use std::io;
enum Operation {
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64)
}

fn calculate(operation: Operation) -> f64{
    match operation{
        Operation::Add(x, y) =>{
           x+y
        }
        Operation::Divide((x), (y)) => {
            if y != 0.0 {
                x / y
            } else {
                println!("Error: Division by zero!");
                std::f64::NAN 
            }
        }
        Operation::Subtract(x, y) => {
           x - y 
        }
        Operation::Multiply(x, y) => {
           x * y     
        }   
    }
}




fn main() {

    
    println!("Hi! Thank you for using our lovely calculator.");
    println!("Please enter you first number.");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number);
    let first_number: f64 = first_number.trim().parse().expect("Invalid input. Please enter a valid number.");;
    println!("Thank you, please type the opearator that you want to use. (add,div,sub,mul)");
    let mut op = String::new();
    io::stdin().read_line(&mut op);
    let op = op.trim();
    println!("Thank you, now please enter the second number.");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number);
    let second_number = second_number.trim().parse().expect("Invalid input. Please enter a valid number.");

    let calculator = match op {
        "add" => Operation::Add(first_number, second_number),
        "sub" => Operation::Subtract(first_number, second_number),
        "mul" => Operation::Multiply(first_number, second_number),
        "div" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation. Please type add, sub, div, or mul.");
            return;
        }
    };
    let result = calculate(calculator);
    println!("The result is: {}", result);

}


