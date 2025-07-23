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
    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^{}", self.val)
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "y:{}", self.val)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HSymbol {
    fn haystack_type(&self) -> HType {
        SYMBOL_TYPE
    }

    set_trait_eq_method!(get_symbol, 'a, T);
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
    fn test_haystack_type() {
        let symbol = HSymbol::new("example".to_string());
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        assert_eq!(symbol_hval.haystack_type(), HType::Symbol);
    }
}
