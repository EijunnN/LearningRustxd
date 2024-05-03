fn main () {

    let coord = (1,2);
    println!("{:?}", coord.0);

    let (x, y) = (1, 2);
    println!("{:?} {:?}", x, y);

    let (name, age) = ("Arturo", 27);
    println!("Tu nombre es {:?}", name);
    println!("Tue dad es {:?}", age);

    // Mal ejemplo  
    let favorites = (12, "blue", "carrito", "casa", "tv show");

    let color = favorites.1;
    print!("{:?}", color);
}