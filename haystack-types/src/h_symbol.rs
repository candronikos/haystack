use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

#[derive(Clone, PartialEq)]
pub struct HSymbol {
    val: String,
}

pub type Symbol = HSymbol;

const SYMBOL_TYPE: HType = HType::Symbol;

impl HSymbol {
    pub fn new(val: String) -> HSymbol {
        HSymbol { val }
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HSymbol {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "^{}", self.val)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "y:{}", self.val)
    }
    fn haystack_type(&self) -> HType {
        SYMBOL_TYPE
    }

    set_trait_eq_method!(get_symbol_val, 'a, T);
    set_get_method!(get_symbol_val, HSymbol);
}

#[cfg(test)]
mod symbol_tests {
    use super::*;

    #[test]
    fn test_new() {
        let symbol = HSymbol::new("example".to_string());
        assert_eq!(symbol.val, "example");
    }

    #[test]
    fn test_to_zinc() {
        let symbol = HSymbol::new("example".to_string());
        let mut buf = String::new();
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        symbol_hval.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "^example");
    }

    #[test]
    fn test_to_trio() {
        let symbol = HSymbol::new("example".to_string());
        let mut buf = String::new();
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        symbol_hval.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "^example");
    }

    #[test]
    fn test_to_json() {
        let symbol = HSymbol::new("example".to_string());
        let mut buf = String::new();
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        symbol_hval.to_json(&mut buf).unwrap();
        assert_eq!(buf, "y:example");
    }

    #[test]
    fn test_haystack_type() {
        let symbol = HSymbol::new("example".to_string());
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        assert_eq!(symbol_hval.haystack_type(), HType::Symbol);
    }
}
