use crate::{HVal,HType};
use crate::common::Txt;
use std::fmt::{self,Write};

#[derive(PartialEq,Debug)]
pub struct HNull;

pub const NULL: HNull = HNull {};

const ZINC: Txt = Txt::Const("N");
const JSON: Txt = Txt::Const("null");

const THIS_TYPE: HType = HType::Null;

impl HVal for HNull {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}