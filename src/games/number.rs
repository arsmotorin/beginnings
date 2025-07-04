use std::io;

pub fn main() {
    println!("Hi! Guess the number.");

    // Store the user's guess
    // in a mutable String variable
    
    // Mutable variables can be changed
    // Immutable variables cannot be changed
    let mut guess = String::new();

    // :: is used to access functions in a module
    // io::stdin() is a function that reads input from the standard input
    // read_line() is a method that reads a line from the input
    // expect() is used to handle errors
    io::stdin()
        
        // & mut guess is a mutable reference to the guess variable
        // This allows us to modify the guess variable
        
        // Without & mut, we would not be able to modify the variable
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    println!("You guessed: {}", guess);
}