


enum Flavors { 
    Sweet,
    Bitter,
    Acid,
    Cool

}

struct StoreDrink {
    flavor : Flavors,
    oz : f32
}


fn flavors_ounces (bebida : StoreDrink) { 
    match bebida.flavor {
        Flavors::Sweet => println!("Tu bebida es dulce"),
        Flavors::Bitter => println!("Tu bebida es amarga"),
        Flavors::Acid => println!("Tu bebida es acida"),
        Flavors::Cool => println!("Tu bebida es fresca")
    }
    println!("Tienes {} onzas ", bebida.oz);

}

fn main () {
    let bebida_dulce = StoreDrink { 
        flavor : Flavors::Sweet,
        oz: 32.50
    };

    flavors_ounces(bebida_dulce);

    let bebida_amarga: StoreDrink = StoreDrink {
        flavor : Flavors::Bitter,
        oz: 30.23
    };

    flavors_ounces(bebida_amarga);

    let bebida_acida : StoreDrink = StoreDrink {
        flavor : Flavors::Acid,
        oz: 100.02
    };

    flavors_ounces(bebida_acida);

}