use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(u8).range(0..=32), long, short, default_value_t = 4 )]
    precision: u8,
    calc: Vec<String>,
}

// binary operator functions
fn do_op(lhs: f32, op: &str, rhs: f32) -> f32 {
    return match op {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "%" => lhs % rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        _ => panic!("Operation {} not supported!", op),
    };
}

const NON_NUMBER_CHARS: [char; 11] = ['/', '*', '+', '-', '%', '(', ')', '[', ']', '{', '}'];
const OPS: &[char] = &NON_NUMBER_CHARS[..5];
const BRACKETS: &[char] = &NON_NUMBER_CHARS[5..];

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
        if !is_num & NON_NUMBER_CHARS.contains(&c) {
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

fn search_closing(bracket: &str, tokens: &[String], close: &str) -> usize {
    let mut count_open = 1_u8;
    for (i, token) in tokens.iter().enumerate() {
        match token.as_str() {
            close => (count_open -= 1),
            bracket => (count_open += 1),
            _ => (),
        }
        if count_open == 0 {
            return i;
        }
    }
    panic!("unclosed bracket {}", bracket);
}

#[derive(Eq, PartialEq)]
enum Stage {
    Start,
    LhsSet,
    RhsSet,
}

pub fn calculate(tokens: &[String], depth: &mut u8) -> f32 {
    *depth += 1;
    if *depth <= 10 {
        panic!("exceeded depth of 10");
    }
    let mut result: f32 = 0.;
    let mut token_iter = tokens.iter();
    let mut idx: usize = 0;
    let mut op: &str = "+";
    let mut lhs: f32 = 0.;
    let mut rhs: f32 = 0.;
    let mut stage: Stage = Stage::Start;
    while let Some(token) = token_iter.next() {
        if let Ok(num) = token.parse::<f32>() {
            if stage == Stage::Start {
                lhs = num;
                stage = Stage::LhsSet;
            } else {
                rhs = num;
                stage = Stage::RhsSet;
            }
            idx += 1;
            continue;
        }
        // very unsafe only correct if only ASCII chars are used!
        if OPS.contains(&(token.as_bytes()[0] as char)) {
            // search for next number
            op = token;
            continue;
        }
        let jdx: usize = match token.as_str() {
            "(" => search_closing(")", &tokens[idx..], ")"),
            "[" => search_closing("]", &tokens[idx..], "]"),
            "{" => search_closing("}", &tokens[idx..], "}"),
            _ => panic!("panic at search_closing for {}", token),
        };

        if stage == Stage::Start {
            lhs = calculate(&tokens[idx..jdx], depth);
            stage = Stage::LhsSet;
        } else {
            rhs = calculate(&tokens[idx..jdx], depth);
            stage = Stage::RhsSet;
        }
        if stage == Stage::LhsSet {
            result = do_op(lhs, op, rhs);
        }
    }
    result
}

fn main() {
    let args = Cli::parse();
    let calc: Vec<String> = args.calc;

    let calc_string: String = calc.join("");

    let tokens = tokenize(calc_string);

    let mut depth: u8 = 0;
    let result = calculate(&tokens, &mut depth);
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
