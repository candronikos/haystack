use num::Float;
use crate::{HType, HVal, NumTrait};
use crate::common::{escape_str};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub struct HStr(pub String);

pub type Str = HStr;

const STR_TYPE: HType = HType::Str;
const XSTR_TYPE: HType = HType::XStr;

impl HStr {
    pub fn new(s: &str) -> Self {
        HStr(s.to_string())
    }

    pub fn into_string(self) -> String {
        let HStr(s) = self;
        s
    }
    pub fn clone_into_string(&self) -> String {
        let HStr(s) = self;
        s.clone()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HStr {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        buf.push('\"');
        self.0.chars().try_for_each(|c| { escape_str(c,buf) })?;
        buf.push('\"');
        Ok(())
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        match self.0.find(":") {
            Some(_) => write!(buf,"s:{}",self.0),
            None => write!(buf,"{}",self.0),
        }?;
        Ok(())
    }
    fn haystack_type(&self) -> HType { STR_TYPE }

    set_trait_eq_method!(get_string_val,'a,T);
    set_get_method!(get_string_val, HStr);
}

#[derive(Debug,PartialEq)]
pub struct XHStr {
    xtype: String,
    xval: HStr,
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for XHStr {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}(",self.xtype)?;
        HVal::<T>::to_zinc(&self.xval, buf)?;
        write!(buf,")")
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{{ \"_kind\": \"xstr\", \"type\": \"{}\", \"val\": ",self.xtype)?;
        HVal::<T>::to_zinc(&self.xval, buf)?;
        write!(buf,"}}")
    }
    fn haystack_type(&self) -> HType { XSTR_TYPE }

    set_trait_eq_method!(get_xstr_val,'a,T);
    set_get_method!(get_xstr_val, XHStr);
}