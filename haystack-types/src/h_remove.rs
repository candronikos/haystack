use num::Float;
use crate::{HType, HVal, NumTrait};
use crate::common::Txt;
use std::fmt::{self,Write,Display};
use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct HRemove;

pub const REMOVE: HRemove = HRemove {};

const ZINC: Txt = Txt::Const("R");
const JSON: Txt = Txt::Const("-:");

const THIS_TYPE: HType = HType::Remove;

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HRemove {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_remove_val,'a,T);
    set_get_method!(get_remove_val, HRemove);
}