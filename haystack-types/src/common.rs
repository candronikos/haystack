use nom::multi::many1;
use nom::bytes::complete::{take_while1};
use nom::character::complete::alphanumeric1;
use nom::combinator::recognize;
use nom::sequence::tuple;
use nom::bytes::complete::take;
use nom::combinator::verify;
use nom::bytes::complete::tag;
use nom::branch::alt;
use num::Float;
use core::str::FromStr;
use std::fmt::{self,Display,Formatter};
use crate::h_val::HVal;
use nom::IResult;

#[derive(Debug,PartialEq)]
pub enum Txt<'a> {
    Const(&'a str),
    Owned(String)
}

impl <'a>Txt<'a> {
    pub fn chars(&self) -> std::str::Chars {
        match self {
            Txt::Const(s) => s.chars(),
            Txt::Owned(s) => s.chars(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Txt::Const(s) => s.len(),
            Txt::Owned(s) => s.len(),
        }
    }

    pub fn find(&self, pat: &str) -> Option<usize> {
        match self {
            Txt::Const(s) => s.find(pat),
            Txt::Owned(s) => s.find(pat),
        }
    }
}

impl <'a>Display for Txt<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Txt::Const(s) => write!(f, "{}", s),
            Txt::Owned(s) => write!(f, "{}", s),
        }
    }
}

pub fn escape_str(c: char, buf: &mut String) -> fmt::Result {
    if c < ' ' || c == '"' || c == '\\' {
        buf.push('\\');
        match c {
            '\n' => buf.push('n'),
            '\r' => buf.push('r'),
            '\t' => buf.push('t'),
            '"' => buf.push('"'),
            '\\' => buf.push('\\'),
            _ => {
                buf.push_str("u00");
                let tmp = std::char::from_u32(0xf).ok_or(fmt::Error)?;
                if c < tmp { buf.push('0') }
                buf.push_str(&format!("{:X}", c.to_digit(16).ok_or(fmt::Error)?));
            }
        };
    } else {
        buf.push(c);
    }
    Ok(())
}

// TODO: Check if '"' and '`' need to be ignored for both or only one of URIs and Strings
pub fn unicode_char(c: char) -> bool {
    c >= 0x20 as char && c != '\\' && c != '"'
}

pub fn id(input: &str) -> IResult<&str,&str> {
    let lower = |c: char| { c>='a' && c<='z' };
    recognize(tuple((
        take_while1(lower),
        many1(alt((alphanumeric1,tag("_"))))
    )))(input)
}

pub fn u_esc_char(input: &str) -> IResult<&str,&str> {
    recognize(tuple((
        tag("\\u"),
        verify(take(4usize),|s: &str| s.chars().all(|c| char::is_digit(c,16)))
    )))(input)
}

pub trait ZincWriter {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result;
}

pub trait ZincReader {
    fn parse<T: 'static + Float + Display + FromStr>(buf: &'static str) -> IResult<&str, Box<dyn HVal>>;
}

pub trait JsonWriter {
    fn to_json(&self, buf: &mut String) -> fmt::Result;
}

pub trait TrioWriter {
    fn to_trio(&self, buf: &mut String) -> fmt::Result;
}