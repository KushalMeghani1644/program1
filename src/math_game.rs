use rand::Rng;
use std::io;

pub fn game() {
    let mut score = 0;
    let mut attempts = 0;
    let mut rng = rand::rng();

    println!("Welcome to the Math Game!");

    while attempts < 3 {
        let num1 = rng.random_range(0..=99);
        let num2 = rng.random_range(0..=99);

        println!("\nEnter operation (+, -, *, /):");
        let mut op = String::new();
        if io::stdin().read_line(&mut op).is_err() {
            eprintln!("Failed to read operation.");
            continue;
        }

        let op = op.trim();
        let valid_ops = ["+", "-", "*", "/"];

        if !valid_ops.contains(&op) {
            println!("Invalid operation. Try again.");
            continue;
        }

        if op == "/" && num2 == 0 {
            println!("Cannot divide by zero. Skipping question.");
            continue;
        }

        // Ask the question
        let correct_answer = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => unreachable!(),
        };

        println!("What is {} {} {}?", num1, op, num2);

        // Loop until a valid number is given
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input.");
                continue;
            }

            match input.trim().parse::<i32>() {
                Ok(answer) => {
                    if answer == correct_answer {
                        println!("Correct!");
                        score += 1;
                    } else {
                        println!("Wrong! Correct answer: {}", correct_answer);
                    }
                    attempts += 1;
                    break;
                }
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                }
            }
        }
    }

    println!("\nGame over! Final score: {} out of {}", score, attempts);
}
