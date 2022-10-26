use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    calc: Vec<String>,
}


fn main() {
    let calc: Vec<String> = Cli::parse().calc;
    
    let calc_string: Str = calc.join("");
    
    println!("Hello, world!");
}
