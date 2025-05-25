use num::Float;
use crate::{io::HBox, HType, HVal, NumTrait};
use std::fmt::{self,Write,Display,Debug};
use crate::h_grid::HGrid;
use std::str::FromStr;

pub struct HRow<'a,T> {
    inner: Vec<Option<HBox<'a,T>>>
}

pub type Row<'a,T> = HRow<'a,T>;

impl <'a,T: NumTrait + 'a>HRow<'a,T> {
    pub fn new(inner: Vec<Option<HBox<'a,T>>>) -> Self {
        Self { inner }
    }

    pub fn get(&'a self, parent: &'a HGrid<'a,T>, key: &str) -> &Option<HBox<'a,T>> {
        match parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                let idx = col_index.get(key);
                match idx {
                    Some(idx) => match self.inner.get(*idx) {
                        Some(res) => res,
                        None => &None
                    },
                    None => &None
                }
            },
            HGrid::Error { dis, errTrace } => {
                panic!("Error: Row in error grid cannot exist")
            },
            HGrid::Empty { .. } => {
                panic!("Error: Row in empty grid cannot exist")
            }
        }
    }

    pub fn has(&'a self, parent: &'a HGrid<'a,T>, key: &str) -> bool {
        match parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                match col_index.get(key) {
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
            },
            HGrid::Error { dis, errTrace } => {
                panic!("Error: Row in error grid cannot exist")
            },
            HGrid::Empty { .. } => {
                panic!("Error: Row in empty grid cannot exist")
            }
        }
    }

    pub fn to_trio(&self, parent: &HGrid<'a,T>, buf: &mut String) -> fmt::Result {
        match parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                if !cols.is_empty() {
                    let mut iter = cols.iter().enumerate().peekable();
                    while let Some((idx,_c)) = iter.next() {
                        match self.inner.get(idx) {
                            Some(v) => match v {
                                Some(v) => v.to_trio(buf),
                                _ => Ok(())
                            },
                            None => Ok(())
                        }?;
                    }
                }
            },
            HGrid::Error { dis, errTrace } => {
                ()
            },
            HGrid::Empty { .. } => {
                ()
            }
        }
        Ok(())
    }

    pub fn to_zinc(&self, parent: &HGrid<'a,T>, buf: &mut String) -> fmt::Result {
        match parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                if !cols.is_empty() {
                    let mut iter = cols.iter().enumerate().peekable();
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
            },
            HGrid::Error { dis, errTrace } => {
                panic!("Error: Row in error grid cannot exist")
            },
            HGrid::Empty { .. } => {
                panic!("Error: Row in empty grid cannot exist")
            }
        }
        Ok(())
    }
}

impl <'a,T: NumTrait + 'a>Debug for HRow<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl <'a,T: NumTrait + 'a>Display for HRow<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "{:?}",self)?;
        write!(f, "}})")?;
        Ok(())
    }
}