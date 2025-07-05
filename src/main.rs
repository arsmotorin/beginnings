mod calculator;
mod games;
mod other;

unsafe extern "C" {
    fn shadowing();
}

fn main() {
    println!("Hello! It's project for learning Rust programming language.");

    let mut input: String = String::new();

    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let adequate_input = input.trim().to_lowercase();

        match adequate_input.as_str() {
            "calculator" => calculator::calc::main(),
            "game" => games::game::main(),
            "number" => games::number::main(),
            "random" => games::random::main(),
            "shadowing" => {
                unsafe {
                    println!("In C language:");
                    shadowing();
                }
                println!("In Rust language:");
                other::shadowing::main()
            },
            "exit" => {
                println!("Exiting the program. Goodbye!");
                break;
            },
            _ => println!("Unknown command: '{}'. Please try again.", adequate_input),
        }
    }
}