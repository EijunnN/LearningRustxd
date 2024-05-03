struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Wendy".to_owned(),
            locker: Some(199),
        },
        Student {
            name: "Jean pierre".to_owned(),
            locker: None,
        },
        Student {
            name: "Roxana".to_owned(),
            locker: Some(20),
        },
    ];

    for student in students {
        let name = student.name;
        println!("Hola {:?}", name);

        match student.locker {
            Some(answer) => println!("El numero de tu casiila es {}", answer),
            None => println!("No tienes casillero"),
        }
    }
}
