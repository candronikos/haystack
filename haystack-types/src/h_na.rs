use num::Float;
use crate::{HType, HVal, NumTrait};
use crate::common::Txt;
use std::fmt::{self,Write,Display};
use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct HNA;

pub const NA: HNA = HNA {};

const ZINC: Txt = Txt::Const("NA");
const JSON: Txt = Txt::Const("z:");

const THIS_TYPE: HType = HType::NA;

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HNA {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_na_val,'a,T);
    set_get_method!(get_na_val, HNA);
}