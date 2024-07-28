fn main () {
    let a : Option<i32> = Some(32);

    println!("{:?}",a);


    let a_is_some = a.is_some();

    println!("{:?}", a_is_some);


    let a_is_none: bool = a.is_none();

    println!("{}", a_is_none);

    let a_mapped = a.map(|num| num * 2);

    println!("{:?}", a_mapped);


    let a_filtered = a.filter(|num | num == &1 );

    println!("{:?}", a_filtered);

    let a_or_else = a.or_else(|| Some(5));

    println!("{:?} xd", a_or_else);

    let a_unwrraped = a.unwrap_or_else(|| 0);

    println!("{:?}", a_unwrraped);
    
    
    
}