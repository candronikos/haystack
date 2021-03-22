use std::collections::HashMap;
use crate::{HVal,HType};
use std::fmt::{self,Write};

#[derive(PartialEq,Debug)]
pub struct HDict {
    inner: HashMap<String, Box<dyn HVal>>
}

pub type Dict = HDict;

const THIS_TYPE: HType = HType::Dict;

impl HDict {
    pub fn new() -> HDict {
        HDict { inner: HashMap::new() }
    }

    pub fn from_map(map: HashMap<String, Box<dyn HVal>>) -> HDict {
        HDict { inner: map }
    }
}

impl HVal for HDict {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{{")?;
        let inner = &self.inner;
        for (k,v) in inner.into_iter() {
            match v.haystack_type() {
                HType::Remove => write!(buf,"-{}",k),
                HType::Marker => write!(buf,"{}",k),
                _ => {
                    write!(buf,"{}:",k)?;
                    v.to_zinc(buf)?;
                    write!(buf,",")
                }
            }?;
        }
        write!(buf,"}}")
    }
    fn to_json(&self, _buf: &mut String) -> fmt::Result {
        unimplemented!()
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}