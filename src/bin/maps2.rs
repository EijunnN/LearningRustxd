// REQUERIMENTS
// Given a user name, create and print out a User struct if the user exists

// Notes*
// - Use the  existing fin_user function to locate a user
// - Ue the map function to create the User
// - Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

impl User {
    fn new(xd: i32, nombre: &str) -> Self {
        Self {
            user_id: xd,
            name: nombre.to_owned(),
        }
    }
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "kattie" => Some(9),
        _ => None,
    }
}
fn main() {
    let user = "xd";
    let create_user = find_user(user).map( |num| User::new(num, user));

    match create_user {
        Some(hola) => println!("{:?}", hola),
        None => println!("No se encontraron datos"),
    }
}
