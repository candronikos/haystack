use std::fmt::Display;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::alpha1;
use nom::combinator::{map, opt, recognize};
use nom::{IResult, Parser};
use haystack_types::{Number as HNumber, NumTrait, Float, Unit as HUnit, Str as HStr};
use haystack_types::io::{self,parse::zinc};

#[derive(Debug,PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    Not,
    Match,
    NotMatch,
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    Increment,
    Decrement,
    Field,
}

impl Operator {
  pub fn as_str(&self) -> &str {
    match self {
        Operator::Add => "+",
        Operator::Subtract => "-",
        Operator::Multiply => "*",
        Operator::Divide => "/",
        Operator::Modulo => "%",
        Operator::Equal => "==",
        Operator::NotEqual => "!=",
        Operator::LessThan => "<",
        Operator::GreaterThan => ">",
        Operator::LessThanOrEqual => "<=",
        Operator::GreaterThanOrEqual => ">=",
        Operator::And => "&&",
        Operator::Or => "||",
        Operator::Not => "!",
        Operator::Match => "~",
        Operator::NotMatch => "!~",
        Operator::Assign => "=",
        Operator::AddAssign => "+=",
        Operator::SubtractAssign => "-=",
        Operator::MultiplyAssign => "*=",
        Operator::DivideAssign => "/=",
        Operator::ModuloAssign => "%=",
        Operator::Increment => "++",
        Operator::Decrement => "--",
        Operator::Field => "$",
    }
  }

  pub fn parse(input: &str) -> nom::IResult<&str, Operator> {
    alt((
      alt((
        map(tag("++"), |_| Operator::Increment),
        map(tag("--"), |_| Operator::Decrement),
      )),
      alt(( // Math operators
        map(tag("+="), |_| Operator::AddAssign),
        map(tag("-="), |_| Operator::SubtractAssign),
        map(tag("*="), |_| Operator::MultiplyAssign),
        map(tag("/="), |_| Operator::DivideAssign),
        map(tag("%="), |_| Operator::ModuloAssign),
        map(tag("+"), |_| Operator::Add),
        map(tag("-"), |_| Operator::Subtract),
        map(tag("*"), |_| Operator::Multiply),
        map(tag("/"), |_| Operator::Divide),
        map(tag("%"), |_| Operator::Modulo),
      )),
      alt(( // Comparison operators
        map(tag("=="), |_| Operator::Equal),
        map(tag("!="), |_| Operator::NotEqual),
        map(tag("<="), |_| Operator::LessThanOrEqual),
        map(tag(">="), |_| Operator::GreaterThanOrEqual),
        map(tag("<"), |_| Operator::LessThan),
        map(tag(">"), |_| Operator::GreaterThan),
      )),
      alt(( // Match operators
        map(tag("!~"), |_| Operator::NotMatch),
        map(tag("~"), |_| Operator::Match),
      )),
      alt(( // Boolean operators
        map(tag("&&"), |_| Operator::And),
        map(tag("||"), |_| Operator::Or),
        map(tag("!"), |_| Operator::Not),
      )),
      map(tag("="), |_| Operator::Assign),
      map(tag("$"), |_| Operator::Field),
    )).parse(input)
  }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Brace {
  CurlyOpen,
  CurlyClose,
  SquareOpen,
  SquareClose,
  RoundOpen,
  RoundClose,
}

impl Brace {
  pub fn as_str(&self) -> &str {
    match self {
      Brace::CurlyOpen => "{",
      Brace::CurlyClose => "}",
      Brace::SquareOpen => "[",
      Brace::SquareClose => "]",
      Brace::RoundOpen => "(",
      Brace::RoundClose => ")",
    }
  }

  pub fn parse(input: &str) -> nom::IResult<&str, Brace> {
    alt((
      map(tag("{"), |_| Brace::CurlyOpen),
      map(tag("}"), |_| Brace::CurlyClose),
      map(tag("["), |_| Brace::SquareOpen),
      map(tag("]"), |_| Brace::SquareClose),
      map(tag("("), |_| Brace::RoundOpen),
      map(tag(")"), |_| Brace::RoundClose),
    ))
    .parse(input)
  }

  pub fn parse_from_type<T: NumTrait>(brace: Brace) -> impl Fn(&str) -> nom::IResult<&str, Token<T>> {
    move |input| map(tag(brace.as_str()), |_| Token::Brace(brace)).parse(input)
  }
}

impl <T: NumTrait>Token<T> {
  pub fn parse(input: &str) -> IResult<&str, Token<T>> {
    alt((
      map(Number::parse, |number| Token::Number(number)),
      map(Str::parse, |string| Token::String(string)),
      map(Name::parse, |name| Token::Name(name)),
      map(Operator::parse, |operator| Token::Operator(operator)),
      map(Brace::parse, |brace| Token::Brace(brace)),
    ))
    .parse(input)
  }
}

pub struct Number<T: NumTrait>(HNumber<T>);

impl <T: NumTrait>Number<T> {
  pub fn new(value: T, unit: Option<HUnit>) -> Self {
    Number(HNumber::new(value, unit))
  }

  pub fn parse(input: &str) -> nom::IResult<&str, HNumber<T>> {
    io::parse::number::<T>(input)
  }
}

pub struct Str;

impl Str {
  pub fn new(value: &str) -> HStr {
    HStr::new(value)
  }

  pub fn parse(input: &str) -> nom::IResult<&str, HStr> {
    zinc::string(input)
  }
}

pub struct Name;

impl Name {
  pub fn new(value: &str) -> String {
    value.to_string()
  }

  pub fn parse(input: &str) -> nom::IResult<&str, String> {
    let head = alt((alpha1, tag("_")));
    let tail = opt(take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'));

    recognize((head,tail))
      .map(|s: &str| s.to_string())
      .parse(input)
  }
}

pub enum Token<T: NumTrait> {
  Number(HNumber<T>),
  String(HStr),
  Name(String),
  Operator(Operator),
  Brace(Brace),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_operator_as_str() {
    assert_eq!(Operator::Add.as_str(), "+");
    assert_eq!(Operator::Subtract.as_str(), "-");
    assert_eq!(Operator::Multiply.as_str(), "*");
    assert_eq!(Operator::Divide.as_str(), "/");
    assert_eq!(Operator::Modulo.as_str(), "%");
    assert_eq!(Operator::Equal.as_str(), "==");
    assert_eq!(Operator::NotEqual.as_str(), "!=");
    assert_eq!(Operator::LessThan.as_str(), "<");
    assert_eq!(Operator::GreaterThan.as_str(), ">");
    assert_eq!(Operator::LessThanOrEqual.as_str(), "<=");
    assert_eq!(Operator::GreaterThanOrEqual.as_str(), ">=");
    assert_eq!(Operator::And.as_str(), "&&");
    assert_eq!(Operator::Or.as_str(), "||");
    assert_eq!(Operator::Not.as_str(), "!");
    assert_eq!(Operator::Match.as_str(), "~");
    assert_eq!(Operator::NotMatch.as_str(), "!~");
    assert_eq!(Operator::Assign.as_str(), "=");
    assert_eq!(Operator::AddAssign.as_str(), "+=");
    assert_eq!(Operator::SubtractAssign.as_str(), "-=");
    assert_eq!(Operator::MultiplyAssign.as_str(), "*=");
    assert_eq!(Operator::DivideAssign.as_str(), "/=");
    assert_eq!(Operator::ModuloAssign.as_str(), "%=");
    assert_eq!(Operator::Increment.as_str(), "++");
    assert_eq!(Operator::Decrement.as_str(), "--");
    assert_eq!(Operator::Field.as_str(), "$");
  }

  #[test]
  fn test_operator_parse() {
    assert_eq!(Operator::parse("+").unwrap().1, Operator::Add);
    assert_eq!(Operator::parse("-").unwrap().1, Operator::Subtract);
    assert_eq!(Operator::parse("*").unwrap().1, Operator::Multiply);
    assert_eq!(Operator::parse("/").unwrap().1, Operator::Divide);
    assert_eq!(Operator::parse("%").unwrap().1, Operator::Modulo);
    assert_eq!(Operator::parse("==").unwrap().1, Operator::Equal);
    assert_eq!(Operator::parse("!=").unwrap().1, Operator::NotEqual);
    assert_eq!(Operator::parse("<").unwrap().1, Operator::LessThan);
    assert_eq!(Operator::parse(">").unwrap().1, Operator::GreaterThan);
    assert_eq!(Operator::parse("<=").unwrap().1, Operator::LessThanOrEqual);
    assert_eq!(Operator::parse(">=").unwrap().1, Operator::GreaterThanOrEqual);
    assert_eq!(Operator::parse("&&").unwrap().1, Operator::And);
    assert_eq!(Operator::parse("||").unwrap().1, Operator::Or);
    assert_eq!(Operator::parse("!").unwrap().1, Operator::Not);
    assert_eq!(Operator::parse("~").unwrap().1, Operator::Match);
    assert_eq!(Operator::parse("!~").unwrap().1, Operator::NotMatch);
    assert_eq!(Operator::parse("=").unwrap().1, Operator::Assign);
    assert_eq!(Operator::parse("+=").unwrap().1, Operator::AddAssign);
    assert_eq!(Operator::parse("-=").unwrap().1, Operator::SubtractAssign);
    assert_eq!(Operator::parse("*=").unwrap().1, Operator::MultiplyAssign);
    assert_eq!(Operator::parse("/=").unwrap().1, Operator::DivideAssign);
    assert_eq!(Operator::parse("%=").unwrap().1, Operator::ModuloAssign);
    assert_eq!(Operator::parse("++").unwrap().1, Operator::Increment);
    assert_eq!(Operator::parse("--").unwrap().1, Operator::Decrement);
  }

  #[test]
  fn test_brace_as_str() {
    assert_eq!(Brace::CurlyOpen.as_str(), "{");
    assert_eq!(Brace::CurlyClose.as_str(), "}");
    assert_eq!(Brace::SquareOpen.as_str(), "[");
    assert_eq!(Brace::SquareClose.as_str(), "]");
    assert_eq!(Brace::RoundOpen.as_str(), "(");
    assert_eq!(Brace::RoundClose.as_str(), ")");
  }

  #[test]
  fn test_brace_parse() {
    assert_eq!(Brace::parse("{").unwrap().1, Brace::CurlyOpen);
    assert_eq!(Brace::parse("}").unwrap().1, Brace::CurlyClose);
    assert_eq!(Brace::parse("[").unwrap().1, Brace::SquareOpen);
    assert_eq!(Brace::parse("]").unwrap().1, Brace::SquareClose);
    assert_eq!(Brace::parse("(").unwrap().1, Brace::RoundOpen);
    assert_eq!(Brace::parse(")").unwrap().1, Brace::RoundClose);
  }

  #[test]
  fn test_token_parse_operator() {
    let input = "+";
    let result = Token::<f64>::parse(input).unwrap().1;
    if let Token::Operator(op) = result {
      assert_eq!(op, Operator::Add);
    } else {
      panic!("Expected Token::Operator");
    }
  }

  #[test]
  fn test_token_parse_brace() {
    let input = "{";
    let result = Token::<f64>::parse(input).unwrap().1;
    if let Token::Brace(brace) = result {
      assert_eq!(brace, Brace::CurlyOpen);
    } else {
      panic!("Expected Token::Brace");
    }
  }

  #[test]
  fn test_token_parse_string() {
    let input = "\"hello\"";
    let result = Token::<f64>::parse(input).unwrap().1;
    if let Token::String(s) = result {
        assert_eq!(s.as_str(), "hello");
    } else {
        panic!("Expected Token::String");
    }
  }

  #[test]
  fn test_token_parse_name() {
    let input = "myVariable";
    let result = Token::<f64>::parse(input).unwrap().1;
    if let Token::Name(name) = result {
        assert_eq!(name, "myVariable");
    } else {
        panic!("Expected Token::Name");
    }
  }
}