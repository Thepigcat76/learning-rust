use std::io;

fn main() {
    println!("uwu");
    let mut n = 0;
    n += 1;
    if n >= 0 {
        println!("amogus")
    }
    // Create a new instance of `std::io::stdin`
    let mut input = String::new();

    // Prompt the user for input
    println!("Please enter your name:");

    // Read the user's input
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Display the user's input
    let trimmed_input = input.trim();

    // Check if the input matches "uwu"
    if trimmed_input == "uwu" {
        println!("You entered uwu!");
    } else {
        // Display the user's input
        println!("Hello, {}!", trimmed_input);
    }
}