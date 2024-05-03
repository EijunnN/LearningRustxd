
fn main () { 
    let numbers = vec![1,2,3];

    match numbers.is_empty() { 
        true => println!("Esta vacia tu huevada"),
        false => println!("Esta con elementos oe")
    }
}