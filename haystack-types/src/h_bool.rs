use crate::{HVal,HType};
use crate::common::Txt;
use std::fmt::{self,Write};
pub struct HBool(bool);

pub type Bool = HBool;

const ZINC_TRUE: Txt = Txt::Const("T");
const ZINC_FALSE: Txt = Txt::Const("F");

const JSON_TRUE: Txt = Txt::Const("true");
const JSON_FALSE: Txt = Txt::Const("false");

const THIS_TYPE: HType = HType::Bool;

impl HVal for HBool {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        match self.0 {
            true => write!(buf,"{}",ZINC_TRUE),
            false => write!(buf,"{}",ZINC_FALSE),
        }
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        match self.0 {
            true => write!(buf,"{}",JSON_TRUE),
            false => write!(buf,"{}",JSON_FALSE),
        }
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}