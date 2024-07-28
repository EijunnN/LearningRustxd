use std::collections::HashMap;

struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "bajo de mierda".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "aaea mano".to_owned(),
        },
    );


    for (k , v ) in lockers { 
        println!("El numero es {:?} y el contenido es {:?}", k, v.content);
    }
}
