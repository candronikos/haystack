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

    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\"')?;
        self.0.chars().try_for_each(|c| zinc_escape_str(c, f))?;
        f.write_char('\"')?;
        Ok(())
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(_) = self.0.find(":") {
            write!(f, "s:")?;
        }
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HStr {
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
}
