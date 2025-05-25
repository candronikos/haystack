use num::Float;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Display,Write};
use core::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct HCoord<T> {
    lat: T,
    long: T
}

pub type Coord<T> = HCoord<T>;

const THIS_TYPE: HType = HType::Coord;

impl <T>HCoord<T> {
    pub fn new(lat: T, long: T) -> HCoord<T> {
        HCoord { lat, long }
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HCoord<T> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"C({},{})",self.lat,self.long)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"c:{},{}",self.lat,self.long)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_coord_val,'a,T);
    set_get_method!(get_coord_val, HCoord<T>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hcoord_new() {
        let coord = HCoord::new(10.5, 20.5);
        assert_eq!(coord.lat, 10.5);
        assert_eq!(coord.long, 20.5);
    }

    #[test]
    fn test_hcoord_to_zinc() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        coord.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "C(10.5,20.5)");
    }

    #[test]
    fn test_hcoord_to_trio() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        coord.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "C(10.5,20.5)");
    }

    #[test]
    fn test_hcoord_to_json() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        coord.to_json(&mut buf).unwrap();
        assert_eq!(buf, "c:10.5,20.5");
    }

    #[test]
    fn test_hcoord_haystack_type() {
        let coord = HCoord::new(10.5, 20.5);
        assert_eq!(coord.haystack_type(), HType::Coord);
    }
}
