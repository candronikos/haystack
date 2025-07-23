use crate::common::Txt;
use crate::{HType, HVal, NumTrait};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct HNA;

pub const NA: HNA = HNA {};

const ZINC: Txt = Txt::Const("NA");
const JSON: Txt = Txt::Const("z:");

const THIS_TYPE: HType = HType::NA;

impl HNA {
    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ZINC)
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", JSON)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HNA {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_na,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

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
