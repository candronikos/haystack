use num::Float;
use crate::{HType, HVal, NumTrait};
use crate::common::{Txt};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct HMarker;

pub const MARKER: HMarker = HMarker {};

const ZINC: Txt = Txt::Const("M");
const JSON: Txt = Txt::Const("m:");

const THIS_TYPE: HType = HType::Marker;

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HMarker {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_marker_val,'a,T);
    set_get_method!(get_marker_val, HMarker);
}