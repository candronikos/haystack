use crate::{HVal,HType};
use std::fmt::{self,Write};

pub struct HCoord {
    lat: f64,
    long: f64
}

pub type Coord = HCoord;

const THIS_TYPE: HType = HType::Coord;

impl HVal for HCoord {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"C({},{})",self.lat,self.long)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"c:{},{}",self.lat,self.long)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}