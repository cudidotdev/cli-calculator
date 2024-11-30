use calculator::{Calculate, Calculator};

fn main() {
    // Get expression from command line arguments, skipping the program name
    let expression = std::env::args().skip(1).next();

    if let Some(expression) = expression {
        // Attempt to calculate the expression
        match Calculate::calculate(expression) {
            Ok(v) => println!("{v}"),   // Print result if successful
            Err(e) => eprintln!("{e}"), // Print error if calculation failed
        }
    } else {
        println!("Insert an expression") // No expression provided
    }
}
