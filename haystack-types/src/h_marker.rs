use crate::{HVal,HType};
use crate::common::{Txt};
use std::fmt::{self,Write};

#[derive(PartialEq,Debug)]
pub struct HMarker;

pub const MARKER: HMarker = HMarker {};

const ZINC: Txt = Txt::Const("M");
const JSON: Txt = Txt::Const("m:");

const THIS_TYPE: HType = HType::Marker;

impl HVal for HMarker {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}