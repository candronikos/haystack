use std::fmt;

use crate::{
    h_bool::HBool, h_coord::HCoord, h_date::HDate, h_datetime::HDateTime, h_dict::HDict,
    h_grid::HGrid, h_list::HList, h_marker::HMarker, h_na::HNA, h_null::HNull, h_number::{HNumber, NumTrait},
    h_ref::HRef, h_remove::HRemove, h_str::HStr, h_symbol::HSymbol, h_time::HTime, h_uri::HUri,
    h_xstr::HXStr,
};

pub trait ZincWriter<'a,T:NumTrait + 'a> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result;
}

macro_rules! impl_zinc_writer {
    ($h_type:ty) => {
        impl<'a, T: NumTrait + 'a> ZincWriter<'a, T> for $h_type {
            fn to_zinc(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_zinc(self,buf)
            }
        }
    };
    ($h_type:ty, $num_trait:ident) => {
        impl<'a, T: $num_trait + 'a> ZincWriter<'a, T> for $h_type {
            fn to_zinc(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_zinc(self,buf)
            }
        }
    };
}

impl_zinc_writer!(HNull);
impl_zinc_writer!(HMarker);
impl_zinc_writer!(HRemove);
impl_zinc_writer!(HNA);
impl_zinc_writer!(HBool);
impl_zinc_writer!(HStr);
impl_zinc_writer!(HXStr);
impl_zinc_writer!(HUri);
impl_zinc_writer!(HDate);
impl_zinc_writer!(HDateTime);
impl_zinc_writer!(HTime);
impl_zinc_writer!(HRef);
impl_zinc_writer!(HSymbol);
impl_zinc_writer!(HCoord<T>, NumTrait);
impl_zinc_writer!(HNumber<T>, NumTrait);
impl_zinc_writer!(HDict<'a, T>, NumTrait);
impl_zinc_writer!(HList<'a, T>, NumTrait);
impl_zinc_writer!(HGrid<'a, T>, NumTrait);