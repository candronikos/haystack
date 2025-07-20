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

pub trait JsonWriter<'a, T: NumTrait + 'a> {
    fn to_json(&self, buf: &mut String) -> fmt::Result;
}

macro_rules! impl_json_writer {
    ($h_type:ty) => {
        impl<'a, T: NumTrait + 'a> JsonWriter<'a, T> for $h_type {
            fn to_json(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_json(self, buf)
            }
        }
    };
    ($h_type:ty, $num_trait:ident) => {
        impl<'a, T: $num_trait + 'a> JsonWriter<'a, T> for $h_type {
            fn to_json(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_json(self, buf)
            }
        }
    };
}

impl_json_writer!(HNull);
impl_json_writer!(HMarker);
impl_json_writer!(HRemove);
impl_json_writer!(HNA);
impl_json_writer!(HBool);
impl_json_writer!(HStr);
impl_json_writer!(HXStr);
impl_json_writer!(HUri);
impl_json_writer!(HDate);
impl_json_writer!(HDateTime);
impl_json_writer!(HTime);
impl_json_writer!(HRef);
impl_json_writer!(HSymbol);
impl_json_writer!(HCoord<T>, NumTrait);
impl_json_writer!(HNumber<T>, NumTrait);
impl_json_writer!(HDict<'a, T>, NumTrait);
impl_json_writer!(HList<'a, T>, NumTrait);
impl_json_writer!(HGrid<'a, T>, NumTrait);
