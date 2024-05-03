
// enum Size { 
//     Big,
//     Small
// }

fn print_msg (tamano:bool) {
    match tamano {
        true => println!("Its big"),
        false => println!("Its Small")
    }
}

fn main () { 
    let number = 150;
    // let size = if number > 100 {
    //     Size::Big
    // } else {
    //     Size::Small
    // };

    let tamano =  number > 100;

    print_msg(tamano);


        
}

