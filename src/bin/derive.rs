#[derive(Debug, Clone, Copy)] 
enum Position {
    Manager, 
    Supervisor,
    Worker
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position : Position,
    work_hours : i64
}

fn main () {
    let me = Employee { 
        position : Position::Worker,
        work_hours : 40
    };

    println!("{:?}", me);
    println!("{:?}", me);
    println!("{:?}", me);
    println!("{:?}", me);
    println!("{:?}", me);
    println!("{:?}", me);
    
}