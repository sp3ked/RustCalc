use std::io;

fn main() {
    println!("Welcome to the Rust calculator!");

    loop {
        // Prompt and read the first number
        println!("Enter the first number:");
        let first_num = get_number_from_user();

        // Prompt and read the second number
        println!("Enter the second number:");
        let second_num = get_number_from_user();

        // Display the menu of operations
        println!("\nChoose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");

        // Read the user's choice
        let choice = get_choice();

        // Perform the selected operation
        match choice {
            1 => {
                let result = add_numbers(first_num, second_num);
                println!("Result: {} + {} = {}", first_num, second_num, result);
            }
            2 => {
                let result = subtract_numbers(first_num, second_num);
                println!("Result: {} - {} = {}", first_num, second_num, result);
            }
            3 => {
                let result = multiply_numbers(first_num, second_num);
                println!("Result: {} * {} = {}", first_num, second_num, result);
            }
            4 => {
                if second_num == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                } else {
                    let result = divide_numbers(first_num, second_num);
                    println!("Result: {} / {} = {}", first_num, second_num, result);
                }
            }
            5 => {
                println!("Exiting the calculator. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }

        println!("\n"); // Add a blank line for better formatting
    }
}

// Function to trim and parse user input into a number
fn get_number_from_user() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse()
        .expect("Invalid number. Please enter a valid numeric value.")
}

// Function to read and parse the user's menu choice
fn get_choice() -> u8 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse()
        .unwrap_or(0) // Default to 0 if parsing fails
}

// Functions for arithmetic operations
fn add_numbers(first: f64, second: f64) -> f64 {
    first + second
}

fn subtract_numbers(first: f64, second: f64) -> f64 {
    first - second
}

fn multiply_numbers(first: f64, second: f64) -> f64 {
    first * second
}

fn divide_numbers(first: f64, second: f64) -> f64 {
    first / second
}

