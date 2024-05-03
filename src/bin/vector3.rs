

fn main () {

    let vectores  =  vec![10,20,30,40];

    for i in &vectores {
        match i {
            30 => println!("Thirty"),
            _ => println!("{}", i)
            
        }
    }

    println!("La cantidad de elementos es : {}", vectores.len());
}