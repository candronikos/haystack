use crate::common::{ZincWriter,JsonWriter,TrioWriter};

use std::fmt::{self,Display,Formatter};

#[derive(Debug)]
pub enum HType {
    Null,
    Marker,
    Remove,
    NA,
    Bool,
    Number,
    Str,
    Uri,
    Ref,
    Date,
    Time,
    DateTime,
    Coord,
    XStr,
    List,
    Dict,
    Grid,
}

impl Display for HType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}",self)
    }
}

pub trait HVal {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result;
    fn to_json(&self, buf: &mut String) -> fmt::Result;
    fn haystack_type(&self) -> HType;
}

impl Display for dyn HVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})",self.haystack_type(),self)
    }
}

impl ZincWriter for dyn HVal {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result { self.to_zinc(buf) }
}

impl Display for dyn ZincWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.to_zinc(&mut buf)?;
        write!(f, "{}", buf)
    }
}

impl JsonWriter for dyn HVal {
    fn to_json(&self, buf: &mut String) -> fmt::Result { self.to_json(buf) }
}

impl Display for dyn JsonWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.to_json(&mut buf)?;
        write!(f, "{}", buf)
    }
}

impl TrioWriter for dyn HVal {
    fn to_trio(&self, buf: &mut String) -> fmt::Result { self.to_zinc(buf) }
}

impl Display for dyn TrioWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.to_trio(&mut buf)?;
        write!(f, "{}", buf)
    }
}