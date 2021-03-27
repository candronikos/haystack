use crate::{HVal,HType};
use std::fmt::{self,Write,Display,Formatter};
use num::Float;
use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct HUnit(String);

impl HUnit {
    pub fn new(unit: String) -> HUnit {
        HUnit(unit)
    }
}

#[derive(PartialEq,Debug)]
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

impl <'a, T: 'a + Float + Display + FromStr>HVal<'a,T> for HNumber<T> {
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

    set_trait_eq_method!(get_number_val,'a,T);
    set_get_method!(get_number_val, HNumber<T>);
}

impl Display for HUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}