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

const OPS: [char; 11] = ['(', ')', '/', '*', '+', '-', '%', '[', ']', '{', '}'];
// takes spaceless String as builds calculation tree
pub fn tokenize(instructions: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut stream = instructions.trim().chars();
    let mut token = String::new();
    let mut is_num = false;
    println!("{}", instructions);

    while let Some(c) = stream.next() {
        println!("next char {}", c);
        println!("num {}", is_num);
        if c == ' ' {
            println!("5");
            continue;
        }
        if ('0'..='9').contains(&c) | (c == '.') {
            token.push(c);
            is_num = true;
            println!("1");
        } else if !is_num & (c == '-') {
            token.push(c);
            is_num = true;
            println!("2");
        } else if is_num {
            // is not a number anymore -> terminate
            is_num = false;
            result.push(token);
            token = String::new();
            println!("3");
        }
        if !is_num & OPS.contains(&c) {
            token.push(c);
            result.push(token);
            token = String::new();
            println!("4");
            continue;
        }
        if token.is_empty() {
            panic!("Encountered illegal char '{}' !", c);
        }
    }
    if !token.is_empty() {
        result.push(token);
    }
    return result;
}

fn main() {
    let args = Cli::parse();
    let calc: Vec<String> = args.calc;

    let calc_string: String = calc.join("");

    let tokens = tokenize(calc_string);

    let result = 1.2345 as f32;
    // print the result with specified precision
    println!("{:.1$}", result, args.precision as usize);
}

#[cfg(test)]
mod tests {
    use super::tokenize;
    macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

    #[test]
    fn test_tokenize_empty() {
        let instructions = String::new();
        let result: Vec<String> = Vec::new();
        assert_eq!(result, tokenize(instructions));
    }

    #[test]
    #[should_panic]
    fn test_tokenize_neg_illegal_char() {
        let instructions = "1.23 / 234e10".to_owned();
        println!("{:#?}", tokenize(instructions));
    }

    #[test]
    fn test_tokenize_number() {
        let instructions = "-167.89".to_owned();
        let result: Vec<String> = vec_of_strings!("-167.89");
        assert_eq!(result, tokenize(instructions));
    }
    #[test]
    fn test_tokenize_string() {
        let instructions = "-1 / 2.45 + 67 % 89".to_owned();
        let result: Vec<String> = vec_of_strings!("-1", "/", "2.45", "+", "67", "%", "89");
        assert_eq!(result, tokenize(instructions));
    }
}
