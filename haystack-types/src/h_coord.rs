use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

#[derive(Clone, PartialEq, Debug)]
pub struct HCoord<T> {
    lat: T,
    long: T,
}

pub type Coord<T> = HCoord<T>;

const THIS_TYPE: HType = HType::Coord;

impl<T: NumTrait> HCoord<T> {
    pub fn new(lat: T, long: T) -> HCoord<T> {
        HCoord { lat, long }
    }
    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C({},{})", self.lat, self.long)
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "c:{},{}", self.lat, self.long)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HCoord<T> {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_coord,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let coord = HCoord::new(10.5, 20.5);
        assert_eq!(coord.lat, 10.5);
        assert_eq!(coord.long, 20.5);
    }

    #[test]
    fn test_haystack_type() {
        let coord = HCoord::new(10.5, 20.5);
        assert_eq!(coord.haystack_type(), HType::Coord);
    }
}
