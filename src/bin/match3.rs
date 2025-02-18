// Topic : Decision making with match

// Program requeriment :
// Display "one", "two", "three", or "other" based on whether
// the value of a variable is 1, 2, 3, or some other number,
// respectively

// Notes : 
// Use a variable set to any integer 
// Use a match expression to determine which message to display 
// Use an underscore (_) to match on any value

fn main () {
    let number = 500;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }
}