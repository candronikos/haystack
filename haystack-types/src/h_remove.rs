use crate::{HVal,HType};
use crate::common::Txt;
use std::fmt::{self,Write};

#[derive(PartialEq,Debug)]
pub struct HRemove;

pub const REMOVE: HRemove = HRemove {};

const ZINC: Txt = Txt::Const("R");
const JSON: Txt = Txt::Const("-:");

const THIS_TYPE: HType = HType::Remove;

impl HVal for HRemove {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}