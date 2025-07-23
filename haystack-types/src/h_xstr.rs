use crate::common::escape_str_no_escape_unicode;
use crate::h_str::HStr;
use crate::{HType, HVal, NumTrait};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct HXStr {
    xtype: String,
    xval: HStr,
}

pub type XStr = HXStr;

const XSTR_TYPE: HType = HType::XStr;

impl HXStr {
    pub fn new(xtype: String, xval: String) -> HXStr {
        HXStr {
            xtype,
            xval: HStr::new(xval),
        }
    }
    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.xtype)?;
        self.xval.to_zinc(f)?;
        write!(f, ")")
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{}:", self.xtype)?;
        self.xval
            .chars()
            .try_for_each(|c| escape_str_no_escape_unicode(c, f))
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HXStr {
    /*
    fn to_json_v4(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{{ \"_kind\": \"xstr\", \"type\": \"{}\", \"val\": ",self.xtype)?;
        HVal::<T>::to_zinc(&self.xval, buf)?;
        write!(f," }}")
    }
    */
    fn haystack_type(&self) -> HType {
        XSTR_TYPE
    }

    set_trait_eq_method!(get_xstr,'a,T);
}

#[cfg(test)]
mod tests {}
