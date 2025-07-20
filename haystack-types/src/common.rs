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

pub fn zinc_escape_str(c: char, buf: &mut String) -> fmt::Result {
    if c < ' ' || c == '"' || c == '\\' || c == '$' {
        buf.push('\\');
        match c {
            '\x08' => buf.push('b'),
            '\x0C' => buf.push('f'),
            '\n' => buf.push('n'),
            '\r' => buf.push('r'),
            '\t' => buf.push('t'),
            '\\' | '\"' | '$' => buf.push(c),
            _ => {
                write!(buf, "u{:04x}", c as usize)?;
            }
        };
    } else {
        buf.push(c);
    }
    Ok(())
}

pub fn escape_str_no_escape_unicode(c: char, buf: &mut String) -> fmt::Result {
    if c < ' ' || c == '"' || c == '\\' {
        buf.push('\\');
        match c {
            '\n' => buf.push('n'),
            '\r' => buf.push('r'),
            '\t' => buf.push('t'),
            '"' => buf.push('"'),
            '\\' => buf.push('\\'),
            _ => {
                write!(buf, "u{:04x}", c as usize)?;
            }
        };
    } else {
        buf.push(c);
    }
    Ok(())
}

pub fn escape_str_escape_unicode(c: char, buf: &mut String) -> fmt::Result {
    if c < ' ' || c == '"' || c == '\\' {
        buf.push('\\');
        match c {
            '\n' => buf.push('n'),
            '\r' => buf.push('r'),
            '\t' => buf.push('t'),
            '"' => buf.push('"'),
            '\\' => buf.push('\\'),
            _ => {
                write!(buf, "u{:04x}", c as usize)?;
            }
        };
    } else {
        if c > '\x7F' {
            write!(buf, "\\u{:04x}", c as usize)?;
        } else {
            buf.push(c);
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

pub trait JsonWriter<'a, T: NumTrait + 'a> {
    fn to_json(&self, buf: &mut String) -> fmt::Result;
}
