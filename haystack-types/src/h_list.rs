use num::Float;
use crate::{HVal,HType};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

pub struct HList<'a,T> {
    inner: Vec<Box<dyn HVal<'a,T> + 'a>>
}

pub type List<'a,T> = HList<'a,T>;

const THIS_TYPE: HType = HType::List;

impl <'a,T:'a + Float + Display + FromStr>HList<'a,T> {
    pub fn new() -> HList<'a,T> {
        HList { inner: Vec::new() }
    }

    pub fn from_vec(vec: Vec<Box<dyn HVal<'a,T> + 'a>>) -> HList<'a,T> {
        HList { inner: vec }
    }
}

impl <'a,T:'a + Float + Display + FromStr>HVal<'a,T> for HList<'a,T> {
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

    fn _eq(&self, other: &dyn HVal<'a,T>) -> bool { false }
    set_get_method!(get_list_val, HList<'a,T>);
}