fn maybe_num() -> Option<i32> {
    Some(23)
}

fn maybe_string() -> Option<String> {
    Some("Hola".to_owned())
}

fn main() {
    let plus_one = maybe_num().map(|a| a + 1);

    println!("{:?}", plus_one)
}
