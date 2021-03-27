use std::str::FromStr;
use core::fmt::Display;
use num::Float;
use crate::{HVal, h_bool::HBool, h_null::HNull, h_na::HNA,
    h_marker::HMarker, h_remove::HRemove, h_number::HNumber,
    h_date::HDate, h_datetime::HDateTime, h_time::HTime,
    h_coord::HCoord, h_str::HStr, h_uri::HUri, h_dict::HDict,
    h_list::HList, h_grid::HGrid};


pub trait HCast<'a,FT,T> 
    where
        FT: 'a + Float + Display + FromStr,
        T: HVal<'a,FT> + ?Sized
    {
    fn get_null(&self) -> Option<&HNull>;
    fn get_marker(&self) -> Option<&HMarker>;
    fn get_remove(&self) -> Option<&HRemove>;
    fn get_na(&self) -> Option<&HNA>;
    fn get_bool(&self) -> Option<&HBool>;
    fn get_string(&self) -> Option<&HStr>;
    fn get_uri(&self) -> Option<&HUri>;
    fn get_number(&self) -> Option<&HNumber<FT>>;
}

impl <'a,FT,T>HCast<'a,FT,T> for Box<T>
    where
        FT: 'a + Float + Display + FromStr,
        T: HVal<'a,FT> + ?Sized,
    {
    fn get_null(&self) -> Option<&HNull> { self.get_null_val() }
    fn get_marker(&self) -> Option<&HMarker> { self.get_marker_val() }
    fn get_remove(&self) -> Option<&HRemove> { self.get_remove_val() }
    fn get_na(&self) -> Option<&HNA> { self.get_na_val() }
    fn get_bool(&self) -> Option<&HBool> { self.get_bool_val() }
    fn get_string(&self) -> Option<&HStr> { self.get_string_val() }
    fn get_uri(&self) -> Option<&HUri> { self.get_uri_val() }
    fn get_number(&self) -> Option<&HNumber<FT>> { self.get_number_val() }
}