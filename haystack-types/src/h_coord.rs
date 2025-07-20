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
    pub fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "C({},{})", self.lat, self.long)
    }
    pub fn to_trio(&self, buf: &mut String) -> fmt::Result {
        Self::to_zinc(self, buf)
    }
    pub fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "c:{},{}", self.lat, self.long)
    }

}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HCoord<T> {
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_trio(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "c:{},{}", self.lat, self.long)
    }
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
    fn test_to_zinc() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        coord.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "C(10.5,20.5)");
    }

    #[test]
    fn test_to_trio() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        coord.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "C(10.5,20.5)");
    }

    #[test]
    fn test_to_json() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        coord.to_json(&mut buf).unwrap();
        assert_eq!(buf, "c:10.5,20.5");
    }

    #[test]
    fn test_haystack_type() {
        let coord = HCoord::new(10.5, 20.5);
        assert_eq!(coord.haystack_type(), HType::Coord);
    }
}
