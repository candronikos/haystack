use num::Float;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Write,Display};
use std::str::FromStr;
use crate::common::escape_str;

#[derive(PartialEq)]
pub struct HRef {
    id: String,
    dis: Option<String>,
}

pub type Ref = HRef;

const THIS_TYPE: HType = HType::Ref;

impl HRef {
    pub fn new(id: String, dis: Option<String>) -> HRef {
        HRef { id, dis }
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HRef {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"@{}",self.id)?;
        match &self.dis {
            Some(dis) => {
                buf.push(' ');
                dis.chars().try_for_each(|c| { escape_str(c,buf) })?;
                Ok(())
            },
            None => Ok(()),
        }
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"r:{}",self.id)?;
        match &self.dis {
            Some(dis) => {
                buf.push(' ');
                dis.chars().try_for_each(|c| { escape_str(c,buf) })?;
                Ok(())
            },
            None => Ok(()),
        }
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_ref_val,'a,T);
    set_get_method!(get_ref_val, HRef);
}