// an attempt of the assingment in the rust language book assignment
use std::io; //uses the standard library for user input and output

fn main() {
    println!("Convert degrees celsius to fahrenheit");
    
    loop {
        println!("Please input temperature in degrees celsius!");
    
        let mut t = String::new(); // makes user input variable mutable and also a new empty string

        io::stdin() //handles user input and handles error in the input
            .read_line(&mut t)
            .expect("failed to read temperature");
        
        
        let t: f64 = match t.trim().parse() { //converts user input from string into floating point and mactches the value with the intended outcome and if not requests for new user input
            Ok(num) => num,
            Err(_) => continue,
        };     
        let t = (t * 9.0 / 5.0 ) + 32.0; // performs numerical operations on user input variable
        println!("the temperature in fahrenhiet is: {}F", t);
        break; //breaks the loop when input is equals to intended outcome
        
    }
    
}
    
    




