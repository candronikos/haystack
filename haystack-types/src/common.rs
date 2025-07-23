use crate::{NumTrait, h_val::HBox};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::alphanumeric1;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::{IResult, Parser};
use std::fmt::Write;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Txt<'a> {
    Const(&'a str),
    Owned(String),
}

impl<'a> Txt<'a> {
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

impl<'a> Display for Txt<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Txt::Const(s) => write!(f, "{}", s),
            Txt::Owned(s) => write!(f, "{}", s),
        }
    }
}

pub fn zinc_escape_str(c: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if c < ' ' || c == '"' || c == '\\' || c == '$' {
        f.write_char('\\')?;
        match c {
            '\x08' => f.write_char('b')?,
            '\x0C' => f.write_char('f')?,
            '\n' => f.write_char('n')?,
            '\r' => f.write_char('r')?,
            '\t' => f.write_char('t')?,
            '\\' | '\"' | '$' => f.write_char(c)?,
            _ => {
                write!(f, "u{:04x}", c as usize)?;
            }
        };
    } else {
        f.write_char(c)?;
    }
    Ok(())
}

pub fn escape_str_no_escape_unicode(c: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if c < ' ' || c == '"' || c == '\\' {
        f.write_char('\\')?;
        match c {
            '\n' => f.write_char('n')?,
            '\r' => f.write_char('r')?,
            '\t' => f.write_char('t')?,
            '"' => f.write_char('"')?,
            '\\' => f.write_char('\\')?,
            _ => {
                write!(f, "u{:04x}", c as usize)?;
            }
        };
    } else {
        f.write_char(c)?;
    }
    Ok(())
}

pub fn escape_str_escape_unicode(c: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if c < ' ' || c == '"' || c == '\\' {
        f.write_char('\\')?;
        match c {
            '\n' => f.write_char('n')?,
            '\r' => f.write_char('r')?,
            '\t' => f.write_char('t')?,
            '"' => f.write_char('"')?,
            '\\' => f.write_char('\\')?,
            _ => {
                write!(f, "u{:04x}", c as usize)?;
            }
        };
    } else {
        if c > '\x7F' {
            write!(f, "\\u{:04x}", c as usize)?;
        } else {
            f.write_char(c)?;
        }
    }
    Ok(())
}

pub fn unicode_char(ex: char) -> impl Fn(char) -> bool {
    move |c| c >= 0x20 as char && c != '\\' && c != ex
}

pub fn id(input: &str) -> IResult<&str, &str> {
    let lower = |c: char| c >= 'a' && c <= 'z';
    recognize((take_while1(lower), many0(alt((alphanumeric1, tag("_")))))).parse(input)
}

pub trait ZincReader<'a, T: NumTrait + 'a> {
    fn parse<'b>(buf: &'b str) -> IResult<&'b str, HBox<'a, T>>
    where
        'a: 'b;
}
