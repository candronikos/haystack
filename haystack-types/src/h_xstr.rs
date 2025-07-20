use crate::common::escape_str_no_escape_unicode;
use crate::h_str::HStr;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

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
    pub fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "{}(", self.xtype)?;
        self.xval.to_zinc(buf)?;
        write!(buf, ")")
    }
    pub fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    pub fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "x:{}:", self.xtype)?;
        self.xval
            .chars()
            .try_for_each(|c| escape_str_no_escape_unicode(c, buf))
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HXStr {
    /*
    fn to_json_v4(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{{ \"_kind\": \"xstr\", \"type\": \"{}\", \"val\": ",self.xtype)?;
        HVal::<T>::to_zinc(&self.xval, buf)?;
        write!(buf," }}")
    }
    */
    fn haystack_type(&self) -> HType {
        XSTR_TYPE
    }

    set_trait_eq_method!(get_xstr,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_zinc() {
        let xhstr = HXStr {
            xtype: "custom".to_string(),
            xval: HStr::new("hello".into()),
        };
        let mut buf = String::new();
        xhstr.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "custom(\"hello\")");
    }

    #[test]
    fn test_to_trio() {
        let xhstr = HXStr {
            xtype: "custom".to_string(),
            xval: HStr::new("hello".into()),
        };
        let mut buf = String::new();
        xhstr.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "custom(\"hello\")");
    }

    #[test]
    fn test_to_json() {
        let xhstr = HXStr {
            xtype: "custom".to_string(),
            xval: HStr::new("hello".into()),
        };
        let mut buf = String::new();
        xhstr.to_json(&mut buf).unwrap();
        assert_eq!(buf, "x:custom:hello");
    }
}
