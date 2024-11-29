use calculator_2::{Calculate, Calculator};

fn main() {
    let expression = std::env::args().skip(1).collect();

    match Calculate::calculate(expression) {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}"),
    }
}
