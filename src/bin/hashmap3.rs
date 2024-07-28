// MI SOLUCION
// use std::collections::HashMap;

// struct Shop {
//     stock: i16,
// }

// fn main() {
//     let mut shop = HashMap::new();

//     shop.insert("Sillas", Shop { stock: 2 });
//     shop.insert("Camas", Shop { stock: 2 });
//     shop.insert("Mesas", Shop { stock: 2 });
//     shop.insert("Sofas", Shop { stock: 0 });

//     let mut counter: i16 = 0;
//     for (k, value) in shop.iter() {
//         match value.stock {
//             0 => println!("{} esta agotado", k),
//             _ => println!("{} - {} en stock", k, value.stock),
//         }

//         match value.stock {
//             0 => counter += 0,
//             _ => counter += 1
//         }
//     }

//     println!("El numero de items en stock es {}", counter)
// }

// SOLUCION DEL PROFE

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();

    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);

    let mut total_stock = 0;

    for (item, qty) in stock.iter() {
        total_stock +=  qty;
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };
        println!("items = {:?}, stock {:?}", item, stock_count);
    }

    println!("total stock = {:?} ", total_stock)
}
