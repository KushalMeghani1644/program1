use std::io;

pub fn calculator() {
    println!("Welcome to the Simple Calculator!");

    let num1 = read_number("Enter first number:");
    let num2 = read_number("Enter second number:");

    let op = loop {
        println!("Enter operation (+, -, *, /):");
        let mut op = String::new();
        if io::stdin().read_line(&mut op).is_err() {
            println!("Failed to read operation.");
            continue;
        }

        let op = op.trim();

        if ["+", "-", "*", "/"].contains(&op) {
            break op.to_string();
        } else {
            println!("Unknown operation. Please enter one of +, -, *, /.");
        }
    };

    let result = match op.as_str() {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("Cannot divide by zero.");
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => None, // This should never happen because of the loop validation
    };

    if let Some(r) = result {
        println!("Result: {}", r);
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please try again."),
        }
    }
}
