use crate::{HVal,HType};
use std::fmt::{self,Display,Write};

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

impl <T: Display>HVal for HCoord<T> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"C({},{})",self.lat,self.long)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"c:{},{}",self.lat,self.long)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}