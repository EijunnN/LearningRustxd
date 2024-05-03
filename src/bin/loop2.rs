

fn main () {
    let mut number: i32 = 1;
    loop { 
        
        println!("El numero es {}", number);
        if number == 4 {
            break
        }
        number = number + 1;
    }
    println!("El loop finalizo gaaaa")
}