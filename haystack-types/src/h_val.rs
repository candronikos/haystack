use crate::common::{ZincWriter,ZincReader,JsonWriter,TrioWriter};
use crate::io::HBox;
use crate::{io, NumTrait};
use std::fmt::{self,Display,Formatter,Debug};
use core::str::FromStr;
use std::rc::Rc;
use num::Float;

use nom::IResult;

use crate::{h_bool::HBool, h_null::HNull, h_na::HNA,
    h_marker::HMarker, h_remove::HRemove, h_number::HNumber,
    h_date::HDate, h_datetime::HDateTime, h_time::HTime,
    h_coord::HCoord, h_str::HStr, h_xstr::HXStr, h_uri::HUri, h_ref::HRef, h_dict::HDict,
    h_list::HList, h_grid::HGrid};

#[derive(Debug,PartialEq)]
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

macro_rules! set_get_method {
    ( $name: ident,$tt: ty ) => {
        fn $name(&self) -> Option<&$tt> { Some(self) }
    };
    ( $name: ident,$tt: ty, $lt: lifetime ) => {
        fn $name(&self) -> Option<&$tt<$lt>> { Some(self) }
    };
}

macro_rules! set_trait_get_method {
    ( $name: ident,$tt: ty ) => {
        fn $name(&self) -> Option<&$tt> { None }
    };
    ( $name:ident, $tt:ident, $lt:lifetime, $t:ty ) => {
        fn $name(&$lt self) -> Option<&$lt $tt<$lt,$t>> { None }
    };
}

macro_rules! set_trait_eq_method {
    ( $get_method: ident, $lt: lifetime, $FT: tt ) => {
        fn _eq(&self, other: &dyn HVal<$lt,$FT>) -> bool {
            if let Some(other_obj) = other.$get_method() {
                return self == other_obj
            }
            return false
        }
    }
}

pub trait HVal<'a,T: NumTrait + 'a> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result;
    fn to_trio(&self, buf: &mut String) -> fmt::Result;
    fn to_json(&self, buf: &mut String) -> fmt::Result;
    fn haystack_type(&self) -> HType;

    fn as_hval(&'a self) -> &'a dyn HVal<'a,T>
    where
        Self: Sized + 'a,
    {
        self as &dyn HVal<T>
    }

    fn to_hbox(self) -> HBox<'a,T>
    where
        Self: Sized + 'a,
    {
        Rc::new(self)
    }

    fn _eq(&self, other: &dyn HVal<'a,T>) -> bool;

    set_trait_get_method!(get_null_val, HNull);
    set_trait_get_method!(get_marker_val, HMarker);
    set_trait_get_method!(get_remove_val, HRemove);
    set_trait_get_method!(get_na_val, HNA);
    set_trait_get_method!(get_bool_val, HBool);
    set_trait_get_method!(get_string_val, HStr);
    set_trait_get_method!(get_xstr_val, HXStr);
    set_trait_get_method!(get_uri_val, HUri);
    set_trait_get_method!(get_coord_val, HCoord<T>);
    set_trait_get_method!(get_datetime_val, HDateTime);
    set_trait_get_method!(get_date_val, HDate);
    set_trait_get_method!(get_time_val, HTime);
    set_trait_get_method!(get_number_val, HNumber<T>);
    set_trait_get_method!(get_ref_val, HRef);
    set_trait_get_method!(get_dict_val, HDict,'a,T);
    set_trait_get_method!(get_list_val, HList,'a,T);
    set_trait_get_method!(get_grid_val, HGrid,'a,T);
}

impl <'a,T: NumTrait + 'a>Display for dyn HVal<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let self_as_hval: &(dyn HVal<'a, T> + 'a) = self;
        write!(f, "{}(",self_as_hval.haystack_type())?;
        let mut buf = String::new();
        HVal::to_zinc(self_as_hval, &mut buf)?;
        write!(f, "{})",buf)
    }
}

impl <'a,T: NumTrait + 'a>Debug for dyn HVal<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self)
    }
}

impl <'a,T: NumTrait + 'a>PartialEq for dyn HVal<'a,T> {
    fn eq(&self, other: &dyn HVal<'a,T>) -> bool {
        // TODO: Implement equality testing for HVal
        if self.haystack_type() == other.haystack_type() {
            return self._eq(other);
        };
        false
    }
}

impl <'a,T: NumTrait + 'a>ZincWriter<'a,T> for dyn HVal<'a,T> + 'a {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result { self.to_zinc(buf) }
}

impl <'a,T: NumTrait + 'a>ZincReader<'a,T> for dyn HVal<'a,T> {
    fn parse<'b>(buf: &'b str) -> IResult<&'b str, HBox<'a,T>>
    where
        'a: 'b
    {
        io::parse::zinc::literal::<T>(buf)
    }
}

impl <'a,T: NumTrait + 'a>Display for dyn ZincWriter<'a,T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buf = String::new();
        self.to_zinc(&mut buf)?;
        write!(f, "{}", buf)
    }
}

impl <'a,T: NumTrait + 'a>JsonWriter<'a,T> for dyn HVal<'a,T> {
    fn to_json(&self, buf: &mut String) -> fmt::Result { self.to_json(buf) }
}

impl <'a,T: NumTrait + 'a>Display for dyn JsonWriter<'a,T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.to_json(&mut buf)?;
        write!(f, "{}", buf)
    }
}

impl <'a,T: NumTrait + 'a>TrioWriter<'a,T> for dyn HVal<'a,T> {
    fn to_trio(&self, buf: &mut String) -> fmt::Result { self.to_zinc(buf) }
}

impl <'a,T: NumTrait + 'a>Display for dyn TrioWriter<'a,T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.to_trio(&mut buf)?;
        write!(f, "{}", buf)
    }
}