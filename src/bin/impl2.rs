enum Colors  { 
    White,
    Black,
    Red,
    Blue
}

impl Colors {
    fn print_color (&self) { 
        match self {
            Colors::White => println!("Color blanco"),
            Colors::Black => println!("Color negro"),
            Colors::Red => println!("Color red"),
            Colors::Blue => println!("Color azul")
        }
    }
}


struct Dimensions {
    height : i32,
    width : i32,
    depth : i32
}

impl Dimensions {
    fn print_dimensions (&self)  { 
        println!("Altura {:?}", &self.height);
        println!("Ancho {:?}", &self.width);
        println!("Profundidad {:?}", &self.depth)
    }
}



struct ShippingBox { 
    dimensions : Dimensions,
    weight : f64,
    color : Colors
}


impl ShippingBox { 

    fn new(dimensions : Dimensions, weight : f64, color : Colors) ->Self {
        Self { 
            dimensions,
            color,
            weight,
        }
    }

    fn print(&self) {
        self.color.print_color();
        self.dimensions.print_dimensions();
        println!("{:?}", self.weight);
    }

    
}

fn main () {
    
    let dimensiones = Dimensions { 
        depth : 200,
        height : 200,
        width : 200
    };
    
    let ropa  = ShippingBox  { 
        dimensions : dimensiones,
        color: Colors::Black,
        weight : 150.3
    };

    ropa.print();

    
}