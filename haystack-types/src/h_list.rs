use num::Float;
use crate::io::HBox;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Write,Display};
use std::str::FromStr;
use std::ops::Index;

pub struct HList<'a,T> {
    inner: Vec<HBox<'a,T>>
}

pub type List<'a,T> = HList<'a,T>;

const THIS_TYPE: HType = HType::List;

impl <'a,T: NumTrait + 'a>HList<'a,T> {
    pub fn new() -> HList<'a,T> {
        HList { inner: Vec::new() }
    }

    pub fn from_vec(vec: Vec<HBox<'a,T>>) -> HList<'a,T> {
        HList { inner: vec }
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HList<'a,T> {
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

impl<'a, T> Index<usize> for HList<'a, T> {
    type Output = HBox<'a, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}