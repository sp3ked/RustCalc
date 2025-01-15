use std::io;

fn main() {
    println!("Welcome to the rust calculator");
    
    let mut input = String::new(); //creating a mutable string to hold user input

    
    io::stdin() //Read user input
        .read_line(&mut input)
        .expect("Could read input try agian");

    let input: f64 = input //convert input into a number (f64)
        .trim() //remove trailign whitespace
        .parse() //Attempt to convert the string to a floating point number
        .expect("Invaild number");
    
    println!("You entered the number: {}", input);


}
