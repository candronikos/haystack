use crate::common::Txt;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

#[derive(Clone, PartialEq, Debug)]
pub struct HRemove;

pub const REMOVE: HRemove = HRemove {};

const ZINC: Txt = Txt::Const("R");
const JSON: Txt = Txt::Const("-:");

const THIS_TYPE: HType = HType::Remove;

impl HRemove {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "{}", ZINC)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "{}", JSON)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HRemove {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_trio(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        self.to_json(buf)
    }
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_remove_val,'a,T);
    set_get_method!(get_remove_val, HRemove);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_zinc() {
        let remove = HRemove;
        let mut buf = String::new();
        remove.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "R");
    }

    #[test]
    fn test_to_trio() {
        let remove = HRemove;
        let mut buf = String::new();
        remove.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "R");
    }

    #[test]
    fn test_to_json() {
        let remove = HRemove;
        let mut buf = String::new();
        remove.to_json(&mut buf).unwrap();
        assert_eq!(buf, "-:");
    }

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
