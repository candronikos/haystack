use crate::common::ZincReader;
use crate::io::write::ZincWriter;
use crate::io::write::json::JsonWritable;
use crate::io::write::trio::TrioWritable;
use crate::io::write::zinc::ZincWritable;
use crate::{HCast, NumTrait, io};
use std::fmt::{self, Debug, Display};
use std::rc::Rc;

use nom::IResult;

#[derive(Debug, PartialEq)]
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
    Symbol,
    Date,
    Time,
    DateTime,
    Coord,
    XStr,
    List,
    Dict,
    Grid,
}

pub type HBox<'a, T> = Rc<dyn HVal<'a, T> + 'a>;

impl Display for HType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! set_trait_eq_method {
    ( $get_method: ident, $lt: lifetime, $FT: tt ) => {
        fn _eq(&self, other: &dyn HVal<$lt, $FT>) -> bool {
            if let Some(other_obj) = other.$get_method() {
                return self == other_obj;
            }
            return false
        }
    };
}

pub trait HVal<'a, T: NumTrait + 'a>:
    HCast<'a, T> + ZincWritable + TrioWritable + JsonWritable
{
    fn haystack_type(&self) -> HType;

    fn as_hval(&'a self) -> &'a dyn HVal<'a, T>
    where
        Self: Sized,
    {
        self as &dyn HVal<T>
    }

    fn to_hbox(self) -> HBox<'a, T>
    where
        Self: Sized + 'static,
    {
        Rc::new(self)
    }

    fn to_owned(&self) -> Self
    where
        Self: Sized + HVal<'a, T> + Clone + 'static,
    {
        self.clone()
    }

    fn _eq(&self, other: &dyn HVal<'a, T>) -> bool;
}

impl<'a, T: NumTrait + 'a> Display for dyn HVal<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.haystack_type(), ZincWriter::new(self))
    }
}

impl<'a, T: NumTrait + 'a> Debug for dyn HVal<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a, T: NumTrait + 'a> PartialEq for dyn HVal<'a, T> {
    fn eq(&self, other: &dyn HVal<'a, T>) -> bool {
        // TODO: Implement equality testing for HVal
        if self.haystack_type() == other.haystack_type() {
            return self._eq(other);
        };
        false
    }
}

impl<'a, T: NumTrait + 'a> ZincReader<'a, T> for dyn HVal<'a, T> {
    fn parse<'b>(buf: &'b str) -> IResult<&'b str, HBox<'a, T>>
    where
        'a: 'b,
    {
        let mut dt_cell = io::ParseHint::default();
        io::parse::zinc::literal::<T>(&mut dt_cell)(buf)
    }
}
