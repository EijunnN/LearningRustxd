


struct MyNumbers { 
    favorite : i32
}

fn main () { 

     let number = vec![
        MyNumbers { favorite : 7},
        MyNumbers { favorite : 11},
        MyNumbers { favorite : 27}
     ];

    for numbersxd in number {
        println!("My favorites numbers : {:?}", numbersxd.favorite)
    }
}