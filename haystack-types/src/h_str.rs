use crate::common::zinc_escape_str;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct HStr(pub String);

pub type Str = HStr;

const STR_TYPE: HType = HType::Str;

impl HStr {
    pub fn new(s: String) -> Self {
        HStr(s)
    }

    pub fn chars(&self) -> std::str::Chars<'_> {
        self.0.chars()
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
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    pub fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        buf.push('\"');
        self.0.chars().try_for_each(|c| zinc_escape_str(c, buf))?;
        buf.push('\"');
        Ok(())
    }
    pub fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    pub fn to_json(&self, buf: &mut String) -> fmt::Result {
        if let Some(_) = self.0.find(":") {
            write!(buf, "s:")?;
        }
        write!(buf, "{}", self.0)?;
        Ok(())
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HStr {
    fn to_trio<'b>(&self, buf: &'b mut String) -> fmt::Result {
        self.to_trio(buf)
    }
    fn to_json<'b>(&self, buf: &'b mut String) -> fmt::Result {
        self.to_json(buf)
    }
    fn haystack_type(&self) -> HType {
        STR_TYPE
    }

    set_trait_eq_method!(get_string,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hstr = HStr::new("hello".into());
        assert_eq!(hstr.as_str(), "hello");
    }

    #[test]
    fn test_into_string() {
        let hstr = HStr::new("hello".into());
        let s = hstr.into_string();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_clone_into_string() {
        let hstr = HStr::new("hello".into());
        let s = hstr.clone_into_string();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_to_zinc() {
        let hstr = HStr::new("hello".into());
        let mut buf = String::new();
        hstr.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "\"hello\"");
    }

    #[test]
    fn test_to_zinc_escaped_chars() {
        //let hstr = HStr::new("\b \f \n \r \t \" \\ $ \u{263A}".into());
        let hstr = HStr::new("\x08 \x0C \n \r \t \" \\ $ \u{263A} ☺".into());
        let mut buf = String::new();
        hstr.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "\"\\b \\f \\n \\r \\t \\\" \\\\ \\$ \u{263A} ☺\"");
    }

    #[test]
    fn test_to_trio() {
        let hstr = HStr::new("hello".into());
        let mut buf = String::new();
        HVal::<f64>::to_trio(&hstr, &mut buf).unwrap();
        assert_eq!(buf, "\"hello\"");
    }

    #[test]
    fn test_to_json() {
        let hstr = HStr::new("hello".into());
        let mut buf = String::new();
        HVal::<f64>::to_json(&hstr, &mut buf).unwrap();
        assert_eq!(buf, "hello");

        let hstr_with_colon = HStr::new("key:value".into());
        let mut buf_with_colon = String::new();
        HVal::<f64>::to_json(&hstr_with_colon, &mut buf_with_colon).unwrap();
        assert_eq!(buf_with_colon, "s:key:value");
    }
}
