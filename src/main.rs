use std::io;

mod math_game {
    use rand::Rng;
    use std::io;

    pub fn game() {
        let mut score = 0;
        let mut attempts = 0;
        let mut rng = rand::rng();

        println!("Welcome to the Math Game!");

        while attempts < 3 {
            // Generate numbers between 0 and 99 (inclusive)
            let num1 = rng.random_range(0..=99);
            let num2 = rng.random_range(0..=99);

            // Ask for operation
            println!("Please enter what operation you want to do (+,-,*,/):");
            let mut op = String::new();
            io::stdin().read_line(&mut op).expect("Failed to read line");

            // Handle the operation
            let op = op.trim();
            match op {
                "+" => handle_operation(num1, num2, "+", &mut score, &mut attempts),
                "-" => handle_operation(num1, num2, "-", &mut score, &mut attempts),
                "*" => handle_operation(num1, num2, "*", &mut score, &mut attempts),
                "/" => {
                    if num2 == 0 {
                        println!("Cannot divide by zero. Try again.");
                        continue;
                    }
                    handle_operation(num1, num2, "/", &mut score, &mut attempts)
                }
                _ => {
                    println!("Invalid operation. Please enter (+,-,*,/). Try again.");
                    continue;
                }
            };
        }

        println!(
            "Game over! Your final score is: {} out of {}",
            score, attempts
        );
    }

    // Helper function to eliminate repetition
    fn handle_operation(num1: i32, num2: i32, op: &str, score: &mut i32, attempts: &mut i32) {
        println!("What is {} {} {}?", num1, op, num2);

        let mut answer_input = String::new();
        io::stdin()
            .read_line(&mut answer_input)
            .expect("Failed to read line");

        // Calculate correct answer
        let correct_answer = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                // Convert to f64 to avoid integer division truncation
                (num1 as f64 / num2 as f64) as i32
            }
            _ => unreachable!(), // We've already validated the operation
        };

        // Parse user's answer
        let answer: i32 = match answer_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                *attempts += 1;
                return;
            }
        };

        // Compare with the correct answer
        if answer == correct_answer {
            println!("Correct!");
            *score += 1;
        } else {
            println!("Wrong! The correct answer is {}", correct_answer);
        }
        *attempts += 1;
    }
}

mod simple_calculator {
    use std::io;

    pub fn calculator() {
        println!("Welcome to the Simple Calculator!");

        let mut num1_str = String::new();
        let mut num2_str = String::new();
        let mut operation = String::new();

        println!("Enter first number:");
        io::stdin()
            .read_line(&mut num1_str)
            .expect("Failed to read line");

        println!("Enter second number:");
        io::stdin()
            .read_line(&mut num2_str)
            .expect("Failed to read line");

        println!("Enter operation (+, -, *, /):");
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        // Parse input strings to f64
        let num1: f64 = match num1_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the first number.");
                return;
            }
        };

        let num2: f64 = match num2_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the second number.");
                return;
            }
        };

        let operation = operation.trim();

        // Match operation and calculate result
        let result = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" if num2 != 0.0 => num1 / num2,
            "/" => {
                println!("Cannot divide by zero");
                return;
            }
            _ => {
                println!("Invalid operation");
                return;
            }
        };

        println!("Result: {}", result);
    }
}

fn main() {
    println!("Choose which program to run:");
    println!("1. Math Game");
    println!("2. Simple Calculator");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => math_game::game(),
        "2" => simple_calculator::calculator(),
        _ => println!("Invalid choice."),
    }
}
