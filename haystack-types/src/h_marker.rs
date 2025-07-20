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

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HMarker {
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

    set_trait_eq_method!(get_marker,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_zinc() {
        let marker = HMarker;
        let mut buf = String::new();
        marker.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "M");
    }

    #[test]
    fn test_to_trio() {
        let marker = HMarker;
        let mut buf = String::new();
        marker.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "M");
    }

    #[test]
    fn test_to_json() {
        let marker = HMarker;
        let mut buf = String::new();
        marker.to_json(&mut buf).unwrap();
        assert_eq!(buf, "m:");
    }

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
