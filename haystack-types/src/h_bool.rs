use num::Float;
use crate::{HVal,HType};
use crate::common::Txt;
use std::fmt::{self,Write,Display};
use std::str::FromStr;

#[derive(PartialEq,Debug,Clone)]
pub struct HBool(pub bool);

pub type Bool = HBool;

const ZINC_TRUE: Txt = Txt::Const("T");
const ZINC_FALSE: Txt = Txt::Const("F");

const JSON_TRUE: Txt = Txt::Const("true");
const JSON_FALSE: Txt = Txt::Const("false");

const THIS_TYPE: HType = HType::Bool;

impl <'a,T:'a + Float + Display + FromStr>HVal<'a,T> for HBool {
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

    set_trait_eq_method!(get_bool_val,'a,T);
    set_get_method!(get_bool_val, HBool);
}