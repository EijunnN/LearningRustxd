enum Discount {
    Percent(i32),
    Flat(i32)
}


struct Ticket { 
    event: String,
    price : i32
}

fn main () { 
    let n = 3;
    match n { 
        3 => println!("three"),
        other => println!("Number : {:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat { 
        Discount::Flat(2) => println!("xd"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (), // esto indica que no retornara nada
    }

    let concert = Ticket { 
        event : "La pollada de Wendy".to_owned(),
        price : 20
    };

    match concert {
        Ticket { price: 20, event } => println!("event @ 50 = {:?}", event),
        Ticket { price, .. } => println!("price {:?}", price)
    }
}