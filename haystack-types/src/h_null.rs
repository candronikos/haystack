use crate::{HType, HVal, NumTrait};
use crate::common::Txt;
use std::fmt::{self,Write};

#[derive(Clone,PartialEq,Debug)]
pub struct HNull;

pub const NULL: HNull = HNull {};

const ZINC: Txt = Txt::Const("N");
const JSON: Txt = Txt::Const("null");

const THIS_TYPE: HType = HType::Null;

impl HNull {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",ZINC)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{}",JSON)
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HNull {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_trio(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        self.to_json(buf)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_null_val,'a,T);
    set_get_method!(get_null_val, HNull);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_zinc() {
        let null = HNull;
        let mut buf = String::new();
        null.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "N");
    }

    #[test]
    fn test_to_trio() {
        let null = HNull;
        let mut buf = String::new();
        null.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "N");
    }

    #[test]
    fn test_to_json() {
        let null = HNull;
        let mut buf = String::new();
        null.to_json(&mut buf).unwrap();
        assert_eq!(buf, "null");
    }

    #[test]
    fn test_haystack_type() {
        let null = HNull;
        let hval_null = HVal::<f64>::as_hval(&null);
        assert_eq!(hval_null.haystack_type(), HType::Null);
    }

    #[test]
    fn test_equality() {
        let null1 = HNull;
        let null2 = HNull;
        assert_eq!(null1, null2);
    }
}