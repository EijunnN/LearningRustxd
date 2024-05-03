struct Survey { 
    q1: Option<String>,
    q2: Option<i32>,
    q3: Option<bool>,
}

fn main () { 

    let response = Survey { 
        q1: Some("A".to_owned()),
        q2: Some(10),
        q3: None
    };

    match response.q1 { 
        Some(answer) => println!("La respuesta  de q1 : {:?}", answer),
        None => println!("No ha respondido.")
    }

    match response.q2 {
        Some(answer) => println!("La respuesta de q2 : {:?}", answer),
        None => println!("No ha respondido.")
    }

    match response.q3 {
        Some(answer) => println!("La respuesta de q3 : {:?}", answer),
        None => println!("Este imbecil sigue sin responder dios mios santo")
    }
}