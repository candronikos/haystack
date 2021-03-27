use num::Float;
use std::collections::HashMap;
use crate::{HVal,HType};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

pub struct HDict<'a,T> {
    inner: HashMap<String, Box<dyn HVal<'a,T> + 'a>>
}

pub type Dict<'a,T> = HDict<'a,T>;

const THIS_TYPE: HType = HType::Dict;

impl <'a,T:'a + Float + Display + FromStr>HDict<'a,T> {
    pub fn new() -> HDict<'a,T> {
        HDict { inner: HashMap::new() }
    }

    pub fn from_map(map: HashMap<String, Box<dyn HVal<'a,T> + 'a>>) -> HDict<'a,T> {
        HDict { inner: map }
    }
}

impl <'a,T:'a + Float + Display + FromStr>HVal<'a,T> for HDict<'a,T> {
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

    fn _eq(&self, other: &dyn HVal<'a,T>) -> bool { false }
    set_get_method!(get_dict_val, HDict<'a,T>);
}