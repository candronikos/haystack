use crate::h_val::HBox;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Write};
use std::ops::Index;

#[derive(Clone)]
pub struct HList<'a,T> {
    inner: Vec<HBox<'a,T>>
}

pub type List<'a,T> = HList<'a,T>;

const THIS_TYPE: HType = HType::List;

impl <'a,T: NumTrait>HList<'a,T> {
    pub fn new() -> HList<'a,T> {
        HList { inner: Vec::new() }
    }

    pub fn from_vec(vec: Vec<HBox<'a,T>>) -> HList<'a,T> {
        HList { inner: vec }
    }
}

impl <'a,T: NumTrait>HVal<'a,T> for HList<'a,T> {
    fn to_zinc<'b >(&self, buf: &'b mut String) -> fmt::Result {
        write!(buf,"[")?;
        let inner = &self.inner;
        let mut elements = inner.into_iter().peekable();

        while let Some(v) = elements.next() {
            let () = v.to_zinc(buf)?;
            if elements.peek().is_some() { write!(buf,",")?; }
        };
        write!(buf,"]")
    }
    fn to_trio<'b >(&self, buf: &'b mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
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

#[cfg(test)]
mod tests {
    use crate::{h_number::HNumber, HCast};

    use super::*;

    #[test]
    fn test_new() {
        let hlist = HList::<f64>::new();
        assert!(hlist.inner.is_empty());
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![HNumber::new(1f64,None).to_hbox(), HNumber::new(2.0,None).to_hbox()];
        let vec2 = vec![HNumber::new(1f64,None).to_hbox(), HNumber::new(3.0,None).to_hbox()];
        let hlist = HList::from_vec(vec.clone());
        assert_eq!(hlist.inner, vec);
        assert_ne!(hlist.inner, vec2);
    }

    #[test]
    fn test_to_zinc() {
        let vec = vec![HNumber::new(1f64,None).to_hbox(), HNumber::new(2f64,None).to_hbox()];
        let hlist = HList::from_vec(vec);
        let mut buf = String::new();
        hlist.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "[1,2]");
    }

    #[test]
    fn test_to_trio() {
        let vec = vec![HNumber::new(1f64,None).to_hbox(), HNumber::new(2f64,None).to_hbox()];
        let hlist = HList::from_vec(vec);
        let mut buf = String::new();
        hlist.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "[1,2]");
    }

    #[test]
    fn test_haystack_type() {
        let hlist: HList<f64> = HList::new();
        assert_eq!(hlist.haystack_type(), HType::List);
    }

    #[test]
    fn test_index() {
        let vec = vec![HNumber::new(1f64,None).to_hbox(), HNumber::new(2f64,None).to_hbox()];
        let hlist = HList::from_vec(vec);
        assert_eq!(hlist[0].get_number().unwrap(), &HNumber::new(1f64,None));
        assert_eq!(hlist[1].get_number().unwrap(), &HNumber::new(2f64,None));
        assert_ne!(hlist[0].get_number().unwrap(), &HNumber::new(2f64,None));
    }

    #[test]
    fn test_eq() {
        let hlist1: HList<f64> = HList::new();
        let hlist2: HList<f64> = HList::new();
        let hlist3: HList<f64> = HList::from_vec(vec![HNumber::new(1f64,None).to_hbox(), HNumber::new(2f64,None).to_hbox()]);
        assert!(!hlist1._eq(&hlist2));
        assert!(!hlist1._eq(&hlist2));
        assert!(!hlist1._eq(&hlist3));
    }
}