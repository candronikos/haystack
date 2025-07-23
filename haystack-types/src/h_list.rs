use crate::h_val::HBox;
use crate::{HType, HVal, NumTrait};
use std::fmt;
use std::ops::Index;

#[derive(Clone)]
pub struct HList<'a, T> {
    inner: Vec<HBox<'a, T>>,
}

pub type List<'a, T> = HList<'a, T>;

const THIS_TYPE: HType = HType::List;

impl<'a, T: NumTrait> HList<'a, T> {
    pub fn new() -> HList<'a, T> {
        HList { inner: Vec::new() }
    }

    pub fn from_vec(vec: Vec<HBox<'a, T>>) -> HList<'a, T> {
        HList { inner: vec }
    }

    pub fn get(&self, index: usize) -> Option<&HBox<'a, T>> {
        self.inner.get(index)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn first(&self) -> Option<&HBox<'a, T>> {
        self.inner.first()
    }

    pub fn last(&self) -> Option<&HBox<'a, T>> {
        self.inner.last()
    }
    pub fn push(&mut self, value: HBox<'a, T>) {
        self.inner.push(value);
    }

    pub fn to_zinc<'b>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let inner = &self.inner;
        let mut elements = inner.into_iter().peekable();

        while let Some(v) = elements.next() {
            let () = v.to_zinc(f)?;
            if elements.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
    pub fn to_trio<'b>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut elements = self.inner.iter().peekable();
        while let Some(v) = elements.next() {
            write!(f, "\"")?;
            v.to_json(f)?;
            write!(f, "\"")?;
            if elements.peek().is_some() {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}

impl<'a, T: NumTrait> HVal<'a, T> for HList<'a, T> {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    fn _eq(&self, _other: &dyn HVal<'a, T>) -> bool {
        false
    }
}

impl<'a, T> Index<usize> for HList<'a, T> {
    type Output = HBox<'a, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::h_number::HNumber;

    use super::*;

    #[test]
    fn test_new() {
        let hlist = HList::<f64>::new();
        assert!(hlist.inner.is_empty());
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![
            HNumber::new(1f64, None).to_hbox(),
            HNumber::new(2.0, None).to_hbox(),
        ];
        let vec2 = vec![
            HNumber::new(1f64, None).to_hbox(),
            HNumber::new(3.0, None).to_hbox(),
        ];
        let hlist = HList::from_vec(vec.clone());
        assert_eq!(hlist.inner, vec);
        assert_ne!(hlist.inner, vec2);
    }

    #[test]
    fn test_haystack_type() {
        let hlist: HList<f64> = HList::new();
        assert_eq!(hlist.haystack_type(), HType::List);
    }

    #[test]
    fn test_index() {
        let vec = vec![
            HNumber::new(1f64, None).to_hbox(),
            HNumber::new(2f64, None).to_hbox(),
        ];
        let hlist = HList::from_vec(vec);
        assert_eq!(hlist[0].get_number().unwrap(), &HNumber::new(1f64, None));
        assert_eq!(hlist[1].get_number().unwrap(), &HNumber::new(2f64, None));
        assert_ne!(hlist[0].get_number().unwrap(), &HNumber::new(2f64, None));
    }

    #[test]
    fn test_eq() {
        let hlist1: HList<f64> = HList::new();
        let hlist2: HList<f64> = HList::new();
        let hlist3: HList<f64> = HList::from_vec(vec![
            HNumber::new(1f64, None).to_hbox(),
            HNumber::new(2f64, None).to_hbox(),
        ]);
        assert!(!hlist1._eq(&hlist2));
        assert!(!hlist1._eq(&hlist2));
        assert!(!hlist1._eq(&hlist3));
    }
}
