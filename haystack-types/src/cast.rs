use crate::{
    NumTrait, h_bool::HBool, h_coord::HCoord, h_date::HDate, h_datetime::HDateTime, h_dict::HDict,
    h_grid::HGrid, h_list::HList, h_marker::HMarker, h_na::HNA, h_null::HNull, h_number::HNumber,
    h_ref::HRef, h_remove::HRemove, h_str::HStr, h_symbol::HSymbol, h_time::HTime, h_uri::HUri,
    h_xstr::HXStr,
};

macro_rules! set_trait_get_method {
    ( $name: ident,$tt: ty ) => {
        fn $name(&self) -> Option<&$tt> {
            None
        }
    };
    ( $name:ident, $tt:ident, $lt:lifetime, $t:ty ) => {
        fn $name(&self) -> Option<&$tt<$lt, $t>> {
            None
        }
    };
}

macro_rules! set_get_method {
    ( $name: ident,$tt: ty ) => {
        fn $name(&self) -> Option<&$tt> {
            Some(self)
        }
    };
    ( $name:ident, $tt:ident, $lt:lifetime, $t:ty ) => {
        fn $name(&self) -> Option<&$tt<$lt, $t>> {
            Some(self)
        }
    };
}

pub trait HCast<'a, T>
where
    T: NumTrait,
{
    set_trait_get_method!(get_null, HNull);
    set_trait_get_method!(get_marker, HMarker);
    set_trait_get_method!(get_remove, HRemove);
    set_trait_get_method!(get_na, HNA);
    set_trait_get_method!(get_bool, HBool);
    set_trait_get_method!(get_string, HStr);
    set_trait_get_method!(get_xstr, HXStr);
    set_trait_get_method!(get_uri, HUri);
    set_trait_get_method!(get_coord, HCoord<T>);
    set_trait_get_method!(get_datetime, HDateTime);
    set_trait_get_method!(get_date, HDate);
    set_trait_get_method!(get_time, HTime);
    set_trait_get_method!(get_number, HNumber<T>);
    set_trait_get_method!(get_ref, HRef);
    set_trait_get_method!(get_symbol, HSymbol);
    set_trait_get_method!(get_dict, HDict,'a,T);
    set_trait_get_method!(get_list, HList,'a,T);
    set_trait_get_method!(get_grid, HGrid,'a,T);
}

impl<'a, T> HCast<'a, T> for HNull
where
    T: NumTrait + 'a,
{
    set_get_method!(get_null, HNull);
}
impl<'a, T> HCast<'a, T> for HMarker
where
    T: NumTrait + 'a,
{
    set_get_method!(get_marker, HMarker);
}
impl<'a, T> HCast<'a, T> for HRemove
where
    T: NumTrait + 'a,
{
    set_get_method!(get_remove, HRemove);
}
impl<'a, T> HCast<'a, T> for HNA
where
    T: NumTrait + 'a,
{
    set_get_method!(get_na, HNA);
}

impl<'a, T> HCast<'a, T> for HBool
where
    T: NumTrait + 'a,
{
    set_get_method!(get_bool, HBool);
}

impl<'a, T> HCast<'a, T> for HStr
where
    T: NumTrait + 'a,
{
    set_get_method!(get_string, HStr);
}

impl<'a, T> HCast<'a, T> for HXStr
where
    T: NumTrait + 'a,
{
    set_get_method!(get_xstr, HXStr);
}

impl<'a, T> HCast<'a, T> for HUri
where
    T: NumTrait + 'a,
{
    set_get_method!(get_uri, HUri);
}

impl<'a, T> HCast<'a, T> for HCoord<T>
where
    T: NumTrait + 'a,
{
    set_get_method!(get_coord, HCoord<T>);
}

impl<'a, T> HCast<'a, T> for HDateTime
where
    T: NumTrait + 'a,
{
    set_get_method!(get_datetime, HDateTime);
}

impl<'a, T> HCast<'a, T> for HDate
where
    T: NumTrait + 'a,
{
    set_get_method!(get_date, HDate);
}

impl<'a, T> HCast<'a, T> for HTime
where
    T: NumTrait + 'a,
{
    set_get_method!(get_time, HTime);
}

impl<'a, T> HCast<'a, T> for HNumber<T>
where
    T: NumTrait + 'a,
{
    set_get_method!(get_number, HNumber<T>);
}

impl<'a, T> HCast<'a, T> for HRef
where
    T: NumTrait + 'a,
{
    set_get_method!(get_ref, HRef);
}

impl<'a, T> HCast<'a, T> for HSymbol
where
    T: NumTrait + 'a,
{
    set_get_method!(get_symbol, HSymbol);
}

impl<'a, T> HCast<'a, T> for HDict<'a, T>
where
    T: NumTrait + 'a,
{
    set_get_method!(get_dict, HDict, 'a, T);
}

impl<'a, T> HCast<'a, T> for HList<'a, T>
where
    T: NumTrait + 'a,
{
    set_get_method!(get_list, HList, 'a, T);
}

impl<'a, T> HCast<'a, T> for HGrid<'a, T>
where
    T: NumTrait + 'a,
{
    set_get_method!(get_grid, HGrid<'a, T>);
}
