use crate::{HVal,HType};
use crate::common::Txt;
use std::fmt::{self,Write};

#[derive(PartialEq,Debug)]
pub struct HNA;

pub const NA: HNA = HNA {};

const ZINC: Txt = Txt::Const("NA");
const JSON: Txt = Txt::Const("z:");

const THIS_TYPE: HType = HType::NA;

impl HVal for HNA {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}