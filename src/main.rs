// use nom::{tag_s, alt, opt, pair, named};
use clap::Parser;
use nom::{
  IResult,
  branch::alt,
  multi::{many0, many1},
  combinator::{opt, recognize},
  sequence::{preceded, terminated, tuple},
  character::complete::{char, one_of, multispace0},
};
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(u8).range(0..=32), long, short, default_value_t = 4 )]
    precision: u8,
    calc: Vec<String>,
}

fn ws<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(f: F) -> impl Parser<&'a str, O, E> {
  delimited(multispace0, f, multispace0)
}

fn float(input: &str) -> IResult<&str, &str> {
  alt((
    // Case one: .42
    recognize(
      tuple((
        char('.'),
        decimal,
        opt(tuple((
          one_of("eE"),
          opt(one_of("+-")),
          decimal
        )))
      ))
    )
    , // Case two: 42e42 and 42.42e42
    recognize(
      tuple((
        decimal,
        opt(preceded(
          char('.'),
          decimal,
        )),
        one_of("eE"),
        opt(one_of("+-")),
        decimal
      ))
    )
    , // Case three: 42. and 42.42
    recognize(
      tuple((
        decimal,
        char('.'),
        opt(decimal)
      ))
    )
  ))(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
  recognize(
    many1(
      terminated(one_of("0123456789"), many0(char('_')))
    )
  )(input)
}

fn main() {
    let args = Cli::parse();
    let calc: Vec<String> = args.calc;

    let calc_string: String = calc.join("");

    let result = 1.2345 as f32;
    // print the result with specified precision
    println!("{:.1$}", result, args.precision as usize);
}

#[cfg(test)]
mod tests {}
