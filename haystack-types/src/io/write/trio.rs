use std::fmt;

use crate::{
    h_bool::HBool,
    h_coord::HCoord,
    h_date::HDate,
    h_datetime::HDateTime,
    h_dict::HDict,
    h_grid::HGrid,
    h_list::HList,
    h_marker::HMarker,
    h_na::HNA,
    h_null::HNull,
    h_number::{HNumber, NumTrait},
    h_ref::HRef,
    h_remove::HRemove,
    h_str::HStr,
    h_symbol::HSymbol,
    h_time::HTime,
    h_uri::HUri,
    h_xstr::HXStr,
};

pub trait TrioWriter<'a, T: NumTrait + 'a> {
    fn to_trio(&self, buf: &mut String) -> fmt::Result;
}

macro_rules! impl_trio_writer {
    ($h_type:ty) => {
        impl<'a, T: NumTrait + 'a> TrioWriter<'a, T> for $h_type {
            fn to_trio(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_trio(self, buf)
            }
        }
    };
    ($h_type:ty, $num_trait:ident) => {
        impl<'a, T: $num_trait + 'a> TrioWriter<'a, T> for $h_type {
            fn to_trio(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_trio(self, buf)
            }
        }
    };
}

impl_trio_writer!(HNull);
impl_trio_writer!(HMarker);
impl_trio_writer!(HRemove);
impl_trio_writer!(HNA);
impl_trio_writer!(HBool);
impl_trio_writer!(HStr);
impl_trio_writer!(HXStr);
impl_trio_writer!(HUri);
impl_trio_writer!(HDate);
impl_trio_writer!(HDateTime);
impl_trio_writer!(HTime);
impl_trio_writer!(HRef);
impl_trio_writer!(HSymbol);
impl_trio_writer!(HCoord<T>, NumTrait);
impl_trio_writer!(HNumber<T>, NumTrait);
impl_trio_writer!(HDict<'a, T>, NumTrait);
impl_trio_writer!(HList<'a, T>, NumTrait);
impl_trio_writer!(HGrid<'a, T>, NumTrait);
