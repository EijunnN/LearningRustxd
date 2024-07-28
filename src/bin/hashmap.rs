use std::collections::HashMap;



fn main () { 
    let mut bajezas = HashMap::new();
    bajezas.insert("Anita", 1000);
    bajezas.insert("Jean pierre", 39);
    bajezas.insert("Jose Mateu", 39);
    bajezas.insert("Shinjijito", 42);

    // match bajezas.get("Jean pierre") {
    //     Some(age) => println!("Jean pierre is {} years old", age),
    //     None => println!("Jean pierre is not in the database.")
    // }

    for (name, age) in &bajezas {   // aca imprimimos la clave y el valor, el orden es aleatorio
        println!("{} is {} years old", name, age);
    }


    for name in bajezas.keys() {  // para imprimir las claves
        println!("{}", name)
    }

    for valores in bajezas.values() { 
        println!("{}", valores)
    }
}
