use crate::{HType, HVal};
use num::Float;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter, Write};
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
pub struct HUnit(String);

impl HUnit {
    pub fn new(unit: String) -> HUnit {
        HUnit(unit)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct HNumber<T: Display> {
    val: T,
    unit: Option<HUnit>,
}

pub type Number<T> = HNumber<T>;
pub trait NumTrait: Float + Display + Debug + FromStr {}
impl<T> NumTrait for T where T: Float + Display + Debug + FromStr {}
//impl<'a,T> NumTrait for T where T: 'a + Float + Display + FromStr {}

const THIS_TYPE: HType = HType::Number;

impl<T: Float + Display> Number<T> {
    pub fn new(num: T, unit: Option<HUnit>) -> Self {
        HNumber { val: num, unit }
    }

    pub fn val(&self) -> T {
        self.val
    }

    pub fn unit(&self) -> &Option<HUnit> {
        &self.unit
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HNumber<T> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        match &self.unit {
            Some(unit) => write!(buf, "{}{}", self.val, unit),
            None => write!(buf, "{}", self.val),
        }
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        match &self.unit {
            Some(unit) => write!(buf, "{} {}", self.val, unit),
            None => write!(buf, "{}", self.val),
        }
    }
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_number_val,'a,T);
    set_get_method!(get_number_val, HNumber<T>);
}

impl Display for HUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let unit = HUnit::new("m".to_string());
        assert_eq!(unit.0, "m");
    }

    #[test]
    fn test_display() {
        let unit = HUnit::new("kg".to_string());
        assert_eq!(unit.to_string(), "kg");
    }

    #[test]
    fn test_new_with_unit() {
        let unit = Some(HUnit::new("m".to_string()));
        let number = HNumber::new(42.0, unit.clone());
        assert_eq!(number.val(), 42.0);
        assert_eq!(number.unit, unit);
    }

    #[test]
    fn test_new_without_unit() {
        let number = HNumber::new(3.14, None);
        assert_eq!(number.val(), 3.14);
        assert!(number.unit.is_none());
    }

    #[test]
    fn test_to_zinc_with_unit() {
        let unit = Some(HUnit::new("m".to_string()));
        let number = HNumber::new(42.0, unit);
        let mut buf = String::new();
        number.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "42m");
    }

    #[test]
    fn test_to_zinc_without_unit() {
        let number = HNumber::new(3.14, None);
        let mut buf = String::new();
        number.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "3.14");
    }

    #[test]
    fn test_to_trio() {
        let unit = Some(HUnit::new("kg".to_string()));
        let number = HNumber::new(100.0, unit);
        let mut buf = String::new();
        number.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "100kg");
    }

    #[test]
    fn test_to_json_with_unit() {
        let unit = Some(HUnit::new("m".to_string()));
        let number = HNumber::new(42.0, unit);
        let mut buf = String::new();
        number.to_json(&mut buf).unwrap();
        assert_eq!(buf, "42 m");
    }

    #[test]
    fn test_to_json_without_unit() {
        let number = HNumber::new(3.14, None);
        let mut buf = String::new();
        number.to_json(&mut buf).unwrap();
        assert_eq!(buf, "3.14");
    }

    #[test]
    fn test_haystack_type() {
        let number = HNumber::new(42.0, None);
        assert_eq!(number.haystack_type(), HType::Number);
    }
}
