use crate::{HVal,HType};
use std::fmt::{self,Write};

#[derive(PartialEq,Debug)]
pub struct HList {
    inner: Vec<Box<dyn HVal>>
}

pub type List = HList;

const THIS_TYPE: HType = HType::List;

impl HList {
    pub fn new() -> HList {
        HList { inner: Vec::new() }
    }

    pub fn from_vec(vec: Vec<Box<dyn HVal>>) -> HList {
        HList { inner: vec }
    }
}

impl HVal for HList {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{{")?;
        let inner = &self.inner;
        for v in inner.into_iter() {
            let () = v.to_zinc(buf)?;
            write!(buf,",")?;
        };
        write!(buf,"}}")
    }
    fn to_json(&self, _buf: &mut String) -> fmt::Result {
        unimplemented!()
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}