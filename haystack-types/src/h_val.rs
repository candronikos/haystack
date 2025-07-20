use crate::common::{JsonWriter, TrioWriter, ZincReader, ZincWriter};
use crate::{io, HCast, NumTrait};
use std::fmt::{self, Debug, Display, Formatter};
use std::rc::{Rc, Weak};

use nom::IResult;

use crate::{
    h_bool::HBool, h_coord::HCoord, h_date::HDate, h_datetime::HDateTime, h_dict::HDict,
    h_grid::HGrid, h_list::HList, h_marker::HMarker, h_na::HNA, h_null::HNull, h_number::HNumber,
    h_ref::HRef, h_remove::HRemove, h_str::HStr, h_symbol::HSymbol, h_time::HTime, h_uri::HUri,
    h_xstr::HXStr,
};

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

pub trait HVal<'a, T: NumTrait + 'a>: HCast<'a,T> {
    fn to_zinc<'b>(&self, buf: &'b mut String) -> fmt::Result;
    fn to_trio<'b>(&self, buf: &'b mut String) -> fmt::Result;
    fn to_json(&self, buf: &mut String) -> fmt::Result;
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
        let self_as_hval: &(dyn HVal<'a, T>) = self;
        write!(f, "{}(", self_as_hval.haystack_type())?;
        let mut buf = String::new();
        HVal::to_zinc(self_as_hval, &mut buf)?;
        write!(f, "{})", buf)
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

impl<'a, T: NumTrait + 'a> ZincWriter<'a, T> for dyn HVal<'a, T> + 'a {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
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

impl<'a, T: NumTrait + 'a> Display for dyn ZincWriter<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buf = String::new();
        self.to_zinc(&mut buf)?;
        write!(f, "{}", buf)
    }
}

impl<'a, T: NumTrait + 'a> JsonWriter<'a, T> for dyn HVal<'a, T> {
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        self.to_json(buf)
    }
}

impl<'a, T: NumTrait + 'a> Display for dyn JsonWriter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.to_json(&mut buf)?;
        write!(f, "{}", buf)
    }
}

impl<'a, T: NumTrait + 'a> TrioWriter<'a, T> for dyn HVal<'a, T> {
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
}

impl<'a, T: NumTrait + 'a> Display for dyn TrioWriter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.to_trio(&mut buf)?;
        write!(f, "{}", buf)
    }
}
