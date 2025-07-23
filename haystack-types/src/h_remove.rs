use crate::common::Txt;
use crate::{HType, HVal, NumTrait};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct HRemove;

pub const REMOVE: HRemove = HRemove {};

const ZINC: Txt = Txt::Const("R");
const JSON: Txt = Txt::Const("-:");

const THIS_TYPE: HType = HType::Remove;

impl HRemove {
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

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HRemove {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_remove,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haystack_type() {
        let remove = HRemove;
        let hval_remove = HVal::<f64>::as_hval(&remove);
        assert_eq!(hval_remove.haystack_type(), HType::Remove);
    }

    #[test]
    fn test_remove_equality() {
        let remove1 = HRemove;
        let remove2 = HRemove;
        assert_eq!(remove1, remove2);
    }
}
