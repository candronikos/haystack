use haystack_types::{NumTrait,Number,Str};
use crate::token::{self,Token};

type TInput<T:NumTrait> = token::Token<T>;
type Stack<T:NumTrait> = Box<dyn Iterator<Item=Object<T>>>;

struct Func<T:NumTrait> {
  func: Box<dyn Fn(Stack<T>) -> Result<Object<T>, EResult<T>>>,
  argc: usize,
}

enum Object<T:NumTrait> {
  Number(Number<T>),
  String(Str),

  Func(Func<T>),
}

enum EvalError<T:NumTrait> {
  InvalidArgumentCount { expected: usize, actual: usize },
  InvalidArgumentType { expected: String, actual: String },
  InvalidFunction { name: String },
}

type EResult<T> = Result<Object<T>, EvalError<T>>;

trait Expr<T:NumTrait>: IntoIterator<Item=Object<T>> {
  fn eval(input: TInput<T>) -> EResult<T>;
}

impl <T:NumTrait>Expr<T> for Func<T> {
  fn eval(args: Iter<Object<T>>) -> Result<Object<T>, _> {
      todo!()
  }
}

mod grammar {
  use super::*;
  use nom::sequence::delimited;
  use crate::token::Brace;

  fn eval<T:NumTrait>(input: TInput<T>) -> bool {
    alt(
      Brace::parse(Brace::RoundOpen),
      token::Expr,
      Brace::parse(Brace::RoundClose),
    )(input)
  }

  fn grouping<T:NumTrait>(input: TInput<T>) -> bool {
    delimited(
      Brace::parse(Brace::RoundOpen),
      token::Expr,
      Brace::parse(Brace::RoundClose),
    )(input)
  }
}