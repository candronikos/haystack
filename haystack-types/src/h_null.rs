use crate::common::Txt;
use crate::{HType, HVal, NumTrait};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct HNull;

pub const NULL: HNull = HNull {};

const ZINC: Txt = Txt::Const("N");
const JSON: Txt = Txt::Const("null");

const THIS_TYPE: HType = HType::Null;

impl HNull {
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

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HNull {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_null,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

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
