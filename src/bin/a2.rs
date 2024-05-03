#![allow(non_snake_case)]


fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result : i32)  {
    
    println!("The result is {:?}" , result);
}

fn main() {
    
    let result = sum(15, 25);
    display_result(result)
}
