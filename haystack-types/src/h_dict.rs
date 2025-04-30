use num::Float;
use std::collections::HashMap;
use crate::io::HBox;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

pub struct HDict<'a,T> {
    inner: HashMap<String, HBox<'a,T>>
}

pub type Dict<'a,T> = HDict<'a,T>;

const THIS_TYPE: HType = HType::Dict;

impl <'a,T: NumTrait + 'a>HDict<'a,T> {
    pub fn new() -> HDict<'a,T> {
        HDict { inner: HashMap::new() }
    }

    pub fn from_map(map: HashMap<String, HBox<'a,T>>) -> HDict<'a,T> {
        HDict { inner: map }
    }

    pub fn get(&self, key: &str) -> Option<&HBox<'a,T>> {
        self.inner.get(key)
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HDict<'a,T> {
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