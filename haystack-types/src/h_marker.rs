use crate::common::Txt;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

#[derive(Clone, PartialEq, Debug)]
pub struct HMarker;

pub const MARKER: HMarker = HMarker {};

const ZINC: Txt = Txt::Const("M");
const JSON: Txt = Txt::Const("m:");

const THIS_TYPE: HType = HType::Marker;

impl HMarker {
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

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HMarker {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_marker,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haystack_type() {
        let marker = HMarker;
        let hval_marker = HVal::<f64>::as_hval(&marker);
        assert_eq!(hval_marker.haystack_type(), HType::Marker);
    }

    #[test]
    fn test_equality() {
        let marker1 = HMarker;
        let marker2 = HMarker;
        assert_eq!(marker1, marker2);
    }
}
