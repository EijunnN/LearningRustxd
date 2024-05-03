enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main () { 
    // secret file  : admin only
    let access_level = Access::Guest;
    let can_access_file = match access_level { 
        Access::Admin => true,
        _ => false
    };

    println!("Tienes acceso? : {:?}", can_access_file);
}