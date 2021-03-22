use crate::{HVal,HType};
use crate::common::{escape_str};
use std::fmt::{self,Write};

#[derive(Debug,PartialEq)]
pub struct HStr(pub String);

pub type Str = HStr;

const THIS_TYPE: HType = HType::Str;

impl HVal for HStr {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        buf.push('\"');
        self.0.chars().try_for_each(|c| { escape_str(c,buf) })?;
        buf.push('\"');
        Ok(())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        match self.0.find(":") {
            Some(_) => write!(buf,"s:{}",self.0),
            None => write!(buf,"{}",self.0),
        }?;
        Ok(())
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}