


struct Person{
    name: String,
    color: String,
    age: i32,
}

fn print_colors_and_name (color: &str, name: &str) { 
    println!("Color : {}", color);
    println!("Nombre : {}", name)
}

fn main () { 

    let personas = vec![
        Person {
            name: "Arturo".to_owned(),
            color: "Azul".to_owned(),
            age: 27
        },
        Person {
            name : String::from("Jean Pierre"),
            color : String::from("Negro"),
            age: 35
        },
        Person {
            name: String::from("Wendy"),
            color: String::from("Rojo"),
            age : 9
        }
    ];


    for persona in personas  {
        if persona.age < 10 {
            print_colors_and_name(&persona.color, &persona.name)
        }
    }
}