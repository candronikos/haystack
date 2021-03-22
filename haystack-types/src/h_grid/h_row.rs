use crate::{HVal,HType};
use std::fmt::{self,Write,Display,Debug};
use crate::h_grid::HGrid;

pub struct HRow {
    inner: Vec<Option<Box<dyn HVal>>>
}

pub type Row = HRow;

impl HRow {
    pub fn new(inner: Vec<Option<Box<dyn HVal>>>) -> Self {
        Self { inner }
    }

    pub fn get(&self, parent: &HGrid, key: &str) -> &Option<Box<dyn HVal>> {
        let idx = parent.col_index.get(key);
        let res = match idx {
            Some(idx) => match self.inner.get(*idx) {
                Some(res) => res,
                None => &None
            },
            None => &None
        };

        res
    }

    pub fn has(&self, parent: &HGrid, key: &str) -> bool {
        match parent.col_index.get(key) {
            Some(idx) => {
                match self.inner.get(*idx) {
                    Some(opt_ref) => match opt_ref {
                        Some(x) => x.haystack_type() == HType::Null,
                        None => false
                    },
                    None => false
                }
            },
            None => false
        }
    }

    pub fn to_zinc(&self, parent: &HGrid, buf: &mut String) -> fmt::Result {
        if !parent.cols.is_empty() {
            let mut iter = parent.cols.iter().enumerate().peekable();
            while let Some((idx,_c)) = iter.next() {
                match self.inner.get(idx) {
                    Some(v) => match v {
                        Some(v) => v.to_zinc(buf),
                        _ => Ok(())
                    },
                    None => Ok(())
                }?;
                if let Some(_) = iter.peek() {
                    write!(buf, ",")?;
                }
            }
        }
        Ok(())
    }
}

impl Debug for HRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl Display for HRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "}})")?;
        Ok(())
    }
}