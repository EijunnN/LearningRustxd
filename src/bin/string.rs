struct LineItem { 
    name : String,
    price : f64,
    
}


fn print_name (name : &str) { 
    println!("{}", name);
}

fn main () { 
    let product =vec![
        LineItem { 
            name : "La gampi de vicenzo".to_owned(),
            price : 10.0,
        },
        LineItem { 
            name : "El culo de cristina".to_owned(),
            price : 12.0,
        }  
    ];

    for item in product {
        print_name(&item.name);
        println!("{}", item.price);
    }
}