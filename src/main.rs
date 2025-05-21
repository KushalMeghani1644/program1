use std::io;

mod math_game;
mod simple_calculator;

fn main() {
    println!("Choose which program to run:");
    println!("1. Math Game");
    println!("2. Simple Calculator");

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_err() {
        println!("Input error.");
        return;
    }

    match choice.trim() {
        "1" => math_game::game(),
        "2" => simple_calculator::calculator(),
        _ => println!("Invalid choice."),
    }
}
