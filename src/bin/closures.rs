

// forma principal de crear funciones
fn add (a: i32, b:i32) -> i32{ 
    a + b
}

fn main () { 

    // 2da forma para crear funciones 
    // let add = | a : i32 , b: i32 | -> i32 {  a + b };

    // otra forma de crear funciones anonimas, mas usada en closures
    let add = |a, b| a + b;
    let suma = add(2, 2);
    println!("{}",suma)
}