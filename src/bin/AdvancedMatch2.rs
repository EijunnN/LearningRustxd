enum Tickets {
    BackStage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    let ticket = vec![
        Tickets::BackStage("Arturo".to_owned(), 100.0),
        Tickets::Vip("Vicenzo".to_owned(), 50.0),
        Tickets::Standard(20.0),
    ];

    for t in ticket {
        match t {
            Tickets::BackStage(name, price) => {
                println!("El nombre del evento {} y el precio es {}", name, price)
            }
            Tickets::Vip(name, price) => {
                println!("El nombre del evento {} y el precio es {}", name, price)
            }
            Tickets::Standard(price) => println!("El precio es {}", price),
        }
    }
}
