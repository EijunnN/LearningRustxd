struct Temperature {
    degrees_f : f64,
}

impl Temperature { 
    fn freezing ()  -> Self { 
        Self {
            degrees_f : 32.0
        }
    }

    fn boiling () -> Self {
        Self { 
            degrees_f : 212.0
        }
    }

    fn show_temp (&self ) { 
        println!("{:?}", &self.degrees_f) ;
    }
}


fn main () { 
     let hot = Temperature {
        degrees_f : 5.7
     };

     hot.show_temp();


     let cold = Temperature::freezing();

     cold.show_temp();

     let boiling = Temperature::boiling();

     boiling.show_temp();

}

