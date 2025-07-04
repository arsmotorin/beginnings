mod calculator;
mod games;

fn main() {
    println!("Hello! It's project for learning Rust programming language.");

    // Type a module name to run the code
    println!("To run the calculator, type 'calculator'.");
    println!("To run the games, type 'games'.");
    println!("To exit, type 'exit'.");
    let mut input: String = String::new();

    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let adequate_input = input.trim().to_lowercase();

        match adequate_input.as_str() {
            "calculator" => calculator::calc::main(),
            "games" => games::game::main(),
            "exit" => {
                println!("Exiting the program. Goodbye!");
                break;
            },
            _ => println!("Unknown command: '{}'. Please try again.", adequate_input),
        }
    }
}