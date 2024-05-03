
enum Direction {
    Left,
    Right,
    Top,
    Down
}
fn main () {
    let go : Direction = Direction::Left;

    match go { 
        Direction::Left => println!("Vamos a la izquierda"),
        Direction::Right => println!("Vamos a la derecha"),
        Direction::Top => println!("Vamos arriba"),
        Direction::Down => println!("Vamos abajo mierda")
    }
}