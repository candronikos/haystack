use crate::common::Txt;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

#[derive(PartialEq, Debug, Clone)]
pub struct HBool(pub bool);

pub type Bool = HBool;

const ZINC_TRUE: Txt = Txt::Const("T");
const ZINC_FALSE: Txt = Txt::Const("F");

const JSON_TRUE: Txt = Txt::Const("true");
const JSON_FALSE: Txt = Txt::Const("false");

const THIS_TYPE: HType = HType::Bool;

impl HBool {
    pub fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        match self.0 {
            true => write!(buf, "{}", ZINC_TRUE),
            false => write!(buf, "{}", ZINC_FALSE),
        }
    }
    pub fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    pub fn to_json(&self, buf: &mut String) -> fmt::Result {
        match self.0 {
            true => write!(buf, "{}", JSON_TRUE),
            false => write!(buf, "{}", JSON_FALSE),
        }
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HBool {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_bool,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_zinc_true() {
        let hbool = HBool(true);
        let mut buf = String::new();
        hbool.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "T");
    }

    #[test]
    fn test_to_zinc_false() {
        let hbool = HBool(false);
        let mut buf = String::new();
        hbool.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "F");
    }

    #[test]
    fn test_to_json_true() {
        let hbool = HBool(true);
        let mut buf = String::new();
        hbool.to_json(&mut buf).unwrap();
        assert_eq!(buf, "true");
    }

    #[test]
    fn test_to_json_false() {
        let hbool = HBool(false);
        let mut buf = String::new();
        hbool.to_json(&mut buf).unwrap();
        assert_eq!(buf, "false");
    }

    #[test]
    fn test_haystack_type() {
        let hbool = HBool(true);
        let hval_type = HVal::<f64>::as_hval(&hbool);
        assert_eq!(hval_type.haystack_type(), HType::Bool);
    }

    #[test]
    fn test_clone() {
        let hbool = HBool(true);
        let cloned = hbool.clone();
        assert_eq!(hbool, cloned);
    }

    #[test]
    fn test_debug_format() {
        let hbool = HBool(true);
        assert_eq!(format!("{:?}", hbool), "HBool(true)");
    }

    #[test]
    fn test_partial_eq() {
        let hbool1 = HBool(true);
        let hbool2 = HBool(true);
        let hbool3 = HBool(false);
        assert_eq!(hbool1, hbool2);
        assert_ne!(hbool1, hbool3);
    }
}
