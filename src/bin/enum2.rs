


enum Colors {
    Black,
    White,
    Red,
    Blue
}

fn display_color (colors:Colors )  {
    

    match colors {
        Colors::Blue => println!("Tu color favorito es Azul"),
        Colors::White => println!("Tu color vaorito es Blanco"),
        Colors::Red => println!("Tu color favorito es rojo"),
        Colors::Black => println!("Tu color favorito es el negro")
    }
}
fn main () {
    let colores : Colors = Colors::Black;

    display_color(colores)
}