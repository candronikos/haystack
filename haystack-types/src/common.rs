use std::fmt::{self,Display,Formatter};

#[derive(Debug)]
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

pub trait ZincWriter {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result;
}

pub trait JsonWriter {
    fn to_json(&self, buf: &mut String) -> fmt::Result;
}

pub trait TrioWriter {
    fn to_trio(&self, buf: &mut String) -> fmt::Result;
}