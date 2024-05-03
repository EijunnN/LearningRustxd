
struct Estadisticas  { 
    fuerza: i32,
    agilidad: i32,
    inteligencia: i32,
    vitalidad: i32,
    destreza: i32
}


fn main () {

    let my_profile = Estadisticas { 
        fuerza: 10,
        agilidad: 15,
        destreza: 20,
        inteligencia: 25,
        vitalidad: 30,
    };

    println!("Tu fuerza es de {}", my_profile.fuerza);
    println!("Tu agilidad es de {}", my_profile.agilidad);
    println!("Tu destreza es de {}",my_profile.destreza);
    println!("Tu inteligencia es de {}", my_profile.inteligencia);
    println!("Tu vitlidad es de {}", my_profile.vitalidad)
}