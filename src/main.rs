use std::io;

fn main() {
    println!("Welcome to the Rust calculator!");

    // Prompt and read the first number
    println!("Enter the first number:");
    let first_num = get_number_from_user();

    // Prompt and read the second number
    println!("Enter the second number:");
    let second_num = get_number_from_user();

    // Display the numbers entered
    println!("First number entered: {}", first_num);
    println!("Second number entered: {}", second_num);
}

// Function to trim and parse user input into a number
fn get_number_from_user() -> f64 {
    let mut input = String::new(); // Create a mutable string to hold user input

    // Read user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Trim and parse the input into a number
    input
        .trim() // Remove trailing whitespace
        .parse() // Attempt to parse the string into a floating-point number
        .expect("Invalid number")
}
