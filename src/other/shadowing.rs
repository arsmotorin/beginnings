pub fn main() {

    let mut x = 5;
    println!("The value of x is: {x}");

    {
        let x = 2; // This shadows the outer x
        println!("The value of x in the inner scope is: {x}");
    }

    x = 6;
    println!("The value of x is: {x}");
}