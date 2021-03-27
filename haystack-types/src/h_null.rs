use num::Float;
use crate::{HVal,HType};
use crate::common::Txt;
use std::fmt::{self,Write,Display};
use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct HNull;

pub const NULL: HNull = HNull {};

const ZINC: Txt = Txt::Const("N");
const JSON: Txt = Txt::Const("null");

const THIS_TYPE: HType = HType::Null;

impl <'a,T:'a + Float + Display + FromStr>HVal<'a,T> for HNull {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_null_val,'a,T);
    set_get_method!(get_null_val, HNull);
}