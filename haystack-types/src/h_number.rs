use crate::{HVal,HType};
use crate::common::{Txt};
use std::fmt::{self,Write,Display,Formatter};

pub struct HUnit<'a>(Txt<'a>);

pub struct HNumber<'a,T: Display> {
    val: T,
    unit: Option<HUnit<'a>>
}

pub type Number<'a,T> = HNumber<'a,T>;

const THIS_TYPE: HType = HType::Number;

impl <'a,T: Display>HVal for HNumber<'a,T> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        match &self.unit {
            Some(unit) =>  write!(buf,"{}{}",self.val,unit),
            None => write!(buf,"{}",self.val),
        }
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        match &self.unit {
            Some(unit) =>  write!(buf,"{} {}",self.val,unit),
            None => write!(buf,"{}",self.val),
        }
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}

impl <'a>Display for HUnit<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}