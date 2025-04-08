use std::io;

fn main() {

    let mut input1 = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");

    // Input for operator
    let mut operator = String::new();
    println!("Enter the operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator = operator.trim();

    // Input for second number
    let mut input2 = String::new();
    println!("Enter the second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    // Perform operation
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Cannot divide by zero!");
                return;
            } else {
                num1 / num2
            }
        },
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}
