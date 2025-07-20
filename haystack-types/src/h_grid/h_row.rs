use crate::HCol;
use crate::h_grid::HGrid;
use crate::{HType, NumTrait, h_val::HBox};
use rpds::Vector;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Write};
use std::rc::Weak;

#[derive(Clone)]
pub struct HRow<'a, T: NumTrait + 'a> {
    col_index: Weak<HashMap<String, usize>>,
    pub cols: Vector<HCol<'a, T>>,
    pub inner: Weak<Vector<Option<HBox<'a, T>>>>,
}

pub type Row<'a, T> = HRow<'a, T>;

impl<'a, T: NumTrait + 'a> HRow<'a, T> {
    pub fn new(
        col_index: Weak<HashMap<String, usize>>,
        cols: Vector<HCol<'a, T>>,
        inner: Weak<Vector<Option<HBox<'a, T>>>>,
    ) -> Self {
        Self {
            col_index,
            cols,
            inner,
        }
    }

    pub fn get(&'a self, key: &str) -> Option<HBox<'a, T>> {
        let col_index = self.col_index.upgrade().unwrap();
        let idx = col_index.get(key);

        if let Some(idx) = idx {
            match self.inner.upgrade().unwrap().get(*idx) {
                Some(res) => res.clone(),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn has(&'a self, key: &str) -> bool {
        let col_index = self.col_index.upgrade().unwrap();

        match col_index.get(key) {
            Some(idx) => match self.inner.upgrade().unwrap().get(*idx) {
                Some(opt_ref) => match opt_ref {
                    Some(x) => x.haystack_type() != HType::Null,
                    None => false,
                },
                None => false,
            },
            None => false,
        }
    }

    pub fn to_zinc<'b>(&self, buf: &'b mut String) -> fmt::Result {
        if !self.cols.is_empty() {
            let mut iter = self.cols.iter().enumerate().peekable();
            while let Some((idx, _c)) = iter.next() {
                match self.inner.upgrade().unwrap().get(idx) {
                    Some(v) => match v {
                        Some(v) => v.to_zinc(buf),
                        _ => Ok(()),
                    },
                    None => Ok(()),
                }?;
                if let Some(_) = iter.peek() {
                    write!(buf, ",")?;
                }
            }
        }

        Ok(())
    }

    pub fn to_trio<'b>(&self, buf: &'b mut String) -> fmt::Result {
        let col_index = self.col_index.upgrade().unwrap();

        if !col_index.is_empty() {
            let mut iter = self.cols.iter().enumerate().peekable();
            while let Some((idx, _c)) = iter.next() {
                match self.inner.upgrade().unwrap().get(idx) {
                    Some(v) => match v {
                        Some(v) => v.to_trio(buf),
                        _ => Ok(()),
                    },
                    None => Ok(()),
                }?;
            }
        }

        Ok(())
    }
}

impl<'a, T: NumTrait> Debug for HRow<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl<'a, T: NumTrait> Display for HRow<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "{:?}", self)?;
        write!(f, "}})")?;
        Ok(())
    }
}
