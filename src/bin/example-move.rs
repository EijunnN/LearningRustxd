enum Light {
    Dull,
    Bright
}

fn display_light (light: &Light) {
    match light {
        Light::Dull => println!("Aburrido"),
        Light::Bright => println!("Brillante")
    }
}

fn main () {
    let light = Light::Dull;

    display_light(&light);
    display_light(&light);
}