use crate::{HType, HVal, NumTrait};
use crate::common::Txt;
use std::fmt::{self,Write};

#[derive(Clone,PartialEq,Debug)]
pub struct HNA;

pub const NA: HNA = HNA {};

const ZINC: Txt = Txt::Const("NA");
const JSON: Txt = Txt::Const("z:");

const THIS_TYPE: HType = HType::NA;

impl HNA {
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

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HNA {
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

    set_trait_eq_method!(get_na_val,'a,T);
    set_get_method!(get_na_val, HNA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_zinc() {
        let na = HNA;
        let mut buf = String::new();
        na.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "NA");
    }

    #[test]
    fn test_to_trio() {
        let na = HNA;
        let mut buf = String::new();
        na.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "NA");
    }

    #[test]
    fn test_to_json() {
        let na = HNA;
        let mut buf = String::new();
        na.to_json(&mut buf).unwrap();
        assert_eq!(buf, "z:");
    }

    #[test]
    fn test_haystack_type() {
        let na = HNA;
        let hval_na = HVal::<f64>::as_hval(&na);
        assert_eq!(hval_na.haystack_type(), HType::NA);
    }

    #[test]
    fn test_equality() {
        let na1 = HNA;
        let na2 = HNA;
        assert_eq!(na1, na2);
    }
}