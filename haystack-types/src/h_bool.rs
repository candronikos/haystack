use crate::{HType, HVal, NumTrait};
use crate::common::Txt;
use std::fmt::{self,Write,Display};

#[derive(PartialEq,Debug,Clone)]
pub struct HBool(pub bool);

pub type Bool = HBool;

const ZINC_TRUE: Txt = Txt::Const("T");
const ZINC_FALSE: Txt = Txt::Const("F");

const JSON_TRUE: Txt = Txt::Const("true");
const JSON_FALSE: Txt = Txt::Const("false");

const THIS_TYPE: HType = HType::Bool;

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HBool {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        match self.0 {
            true => write!(buf,"{}",ZINC_TRUE),
            false => write!(buf,"{}",ZINC_FALSE),
        }
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
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