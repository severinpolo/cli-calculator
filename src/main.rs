use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(u8).range(0..=32), long, short, default_value_t = 4 )]
    precision: u8,
    calc: Vec<String>,
}

// binary operator functions
fn mult(a: f32, b: f32) -> f32 {
    a * b
}

fn div(a: f32, b: f32) -> f32 {
    a / b
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn subt(a: f32, b: f32) -> f32 {
    a - b
}

fn modulo(a: f32, b: f32) -> f32 {
    a % b
}

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
// takes spaceless String as builds calculation tree
fn calculate(instructions: String) -> f32 {

    let mut a: f32;
    let mut a_String = String::new();
    let mut operation: Operations;
    for c in instructions.chars() {

        match c {
            '0'..='9' => a_String.push(c),
            '*' => operation = Operations::Multiply,
            '/' => operation = Operations::Divide,
            '+' => operation = Operations::Add,
            '-' => operation = Operations::Subtract,
            '%' => operation = Operations::Modulo,
            _ => panic!("Unknown character '{}'!", c)
        }
    }


    return 1.234 as f32;
}

fn main() {
    let args = Cli::parse();
    let calc: Vec<String> = args.calc;

    let calc_string: String = calc.join("");
    
    let result: f32 = calculate(calc_string);
    // print the result with specified precision
    println!("{:.1$}", result, args.precision as usize);
    
}
