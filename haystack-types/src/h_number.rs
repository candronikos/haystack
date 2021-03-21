use crate::{HVal,HType};
use std::fmt::{self,Write,Display,Formatter};
use num::Float;

#[derive(PartialEq,Debug)]
pub struct HUnit(String);

#[derive(PartialEq,)]
pub struct HNumber<T: Display> {
    val: T,
    unit: Option<HUnit>
}

pub type Number<T> = HNumber<T>;

const THIS_TYPE: HType = HType::Number;

impl <T: Float + Display>Number<T> {
    pub fn new(num: T, unit: Option<HUnit>) -> Self {
        HNumber { val: num, unit }
    }
}

impl <T: Display>HVal for HNumber<T> {
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

impl Display for HUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}