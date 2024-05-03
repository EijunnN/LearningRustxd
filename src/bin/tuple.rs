enum Access {
    Full,
}


fn one_two_trhree () -> (i32, i32, i32) {
      (1,2,3)  
}

fn main () {
    let number = one_two_trhree();
    let (x, y, z) = number;

    println!("{:?} {:?}", x, number.0);
    println!("{:?} {:?}", y, number.1);
    println!("{:?} {:?}", z, number.2);

    let (employeed, _access) = ("Vicenzo", Access::Full);

    println!("{:?}", employeed);
    
}