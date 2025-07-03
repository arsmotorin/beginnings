pub fn main() {
    let x = 5;
    let y = 0;
    
    // Sum
    println!("{}", x + y);
    
    // Difference
    println!("{}", x - y);
    
    // Multiplication
    println!("{}", x * y);
    
    // Division and of course, handling division by zero
    if y == 0 {
        println!("Cannot divide by zero, stupid asshole!");
    } else {
        println!("{}", x / y);
    }
    
    // Just printing the values as placeholders
    println!("x: {}, y: {}", x, y);
}