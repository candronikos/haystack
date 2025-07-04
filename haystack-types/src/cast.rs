use crate::{h_bool::HBool, h_coord::HCoord, h_date::HDate, h_datetime::HDateTime, h_dict::HDict, h_grid::HGrid, h_list::HList, h_marker::HMarker, h_na::HNA, h_null::HNull, h_number::HNumber, h_ref::HRef, h_remove::HRemove, h_str::HStr, h_time::HTime, h_uri::HUri, h_val::HBox, HVal, NumTrait};


pub trait HCast<'a,FT> 
    where
        FT: NumTrait,
        //T: HVal<'a,FT> + ?Sized
    {
    fn get_null(&self) -> Option<&HNull>;
    fn get_marker(&self) -> Option<&HMarker>;
    fn get_remove(&self) -> Option<&HRemove>;
    fn get_na(&self) -> Option<&HNA>;
    fn get_bool(&self) -> Option<&HBool>;
    fn get_string(&self) -> Option<&HStr>;
    fn get_uri(&self) -> Option<&HUri>;
    fn get_coord(&self) -> Option<&HCoord<FT>>;
    fn get_number(&self) -> Option<&HNumber<FT>>;
    fn get_datetime(&self) -> Option<&HDateTime>;
    fn get_date(&self) -> Option<&HDate>;
    fn get_time(&self) -> Option<&HTime>;
    fn get_ref(&self) -> Option<&HRef>;
    fn get_dict(&'a self) -> Option<&'a HDict<'a,FT>>;
    fn get_list(&'a self) -> Option<&'a HList<'a,FT>>;
    fn get_grid(&'a self) -> Option<&'a HGrid<'a,FT>>;
}

impl <'a,FT>HCast<'a,FT> for HBox<'a,FT>
    where
        FT: NumTrait + 'a,
    {
    fn get_null(&self) -> Option<&HNull> { self.get_null_val() }
    fn get_marker(&self) -> Option<&HMarker> { self.get_marker_val() }
    fn get_remove(&self) -> Option<&HRemove> { self.get_remove_val() }
    fn get_na(&self) -> Option<&HNA> { self.get_na_val() }
    fn get_bool(&self) -> Option<&HBool> { self.get_bool_val() }
    fn get_string(&self) -> Option<&HStr> { self.get_string_val() }
    fn get_uri(&self) -> Option<&HUri> { self.get_uri_val() }
    fn get_coord(&self) -> Option<&HCoord<FT>> { self.get_coord_val() }
    fn get_number(&self) -> Option<&HNumber<FT>> { self.get_number_val() }
    fn get_datetime(&self) -> Option<&HDateTime> { self.get_datetime_val() }
    fn get_date(&self) -> Option<&HDate> { self.get_date_val() }
    fn get_time(&self) -> Option<&HTime> { self.get_time_val() }
    fn get_ref(&self) -> Option<&HRef> { self.get_ref_val() }
    fn get_dict(&'a self) -> Option<&'a HDict<'a,FT>> { self.get_dict_val() }
    fn get_list(&'a self) -> Option<&'a HList<'a,FT>> { self.get_list_val() }
    fn get_grid(&'a self) -> Option<&'a HGrid<'a,FT>> { self.get_grid_val() }
}