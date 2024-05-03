// Topic : Decision making with match

// Program requeriments :
// Display  "it's true" or "it's false "based on the value a boolean variable

//Notes : 
// Use a variable set to either true or false 
// Use a match expression to determine which message to display

fn main () {
    let is_bool : bool = true;
    match is_bool { 
        true => println!("It's true"),
        false => println!("It's false")
    }
}