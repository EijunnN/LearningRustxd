#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new(name: &str, age: i32) -> Result<Self, String> {
        if age >= 21 {
            Ok(Self {
                name: name.to_owned(),
                age,
            })
        } else {
            Err("No tienes 21 o mas de edad".to_owned())
        }
    }
}

fn print_adult(adult: Result<Adult, String>) {
    match adult {
        Ok(ok) => println!("Hola tu nombre es {:?} y tu edad es {:?}", ok.name, ok.age),
        Err(e) => println!("Errores de la vida {:?}", e),
    }
}

fn main() {
    let wendy = Adult::new("Wendy", 13);
    print_adult(wendy)
}
