use crate::{HVal,HType};
use std::fmt::{self,Write};
use crate::common::{Txt,escape_str};
pub struct HRef<'a> {
    id: Txt<'a>,
    dis: Option<Txt<'a>>,
}

pub type Ref<'a> = HRef<'a>;

const THIS_TYPE: HType = HType::Ref;

impl <'a>HVal for HRef<'a> {
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
}