struct GroceryItem { 
    id: i32,
    quantity: i32
}


fn display_id (grocery : GroceryItem) {
    println!("id = {:?}", grocery.id );
}

fn display_quantity (grocery : GroceryItem) {
    println!("quantity = {:?}", grocery.quantity );
}


fn main () {

    let grocery = GroceryItem { 
        id: 1,
        quantity: 1000
    }; 

    display_id(grocery);
    println!("Holaaaaaa")
    
    // display_quantity(&grocery);
}