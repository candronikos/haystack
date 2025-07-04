use crate::{h_dict::HDict, h_val::HBox, HType, NumTrait};
use std::{fmt::{self, Debug, Display, Write}};
use std::rc::Weak;
use crate::h_grid::HGrid;
use rpds::Vector;

#[derive(Clone)]
pub struct HRow<'a,T:NumTrait + 'a> {
    //parent: Option<Rc<HGrid<'a,T>>>,
    parent: HGrid<'a,T>,
    inner: Weak<Vector<Option<HBox<'a,T>>>>
}

pub type Row<'a,T> = HRow<'a,T>;

impl <'a,T: NumTrait + 'a>HRow<'a,T> {
    pub fn new(parent: HGrid<'a,T>, inner: Weak<Vector<Option<HBox<'a,T>>>>) -> Self {
        Self { parent, inner }
    }

    #[cfg(feature = "lua")]
    pub fn to_dict(self) -> HDict<'a,T> {
        let mut dict = HDict::new();
        match &self.parent {
            HGrid::Grid { cols, .. } => {
                for (idx, col) in cols.iter().enumerate() {
                    let inner = &self.inner;
                    if let Some(val) = inner.upgrade().unwrap().get(idx) {
                        if let Some(v) = val {
                            dict.set(col.name.to_owned(), v.clone());
                        }
                    }
                }
            },
            HGrid::Error { .. } => {
                panic!("Error: Row in error grid cannot exist")
            },
            HGrid::Empty { .. } => {
                panic!("Error: Row in empty grid cannot exist")
            }
        }
        dict
    }

    pub fn get(&'a self, key: &str) -> Option<HBox<'a,T>> {
        match &self.parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                let idx = col_index.get(key);
                
                if let Some(idx) = idx {
                    match self.inner.upgrade().unwrap().get(*idx) {
                        Some(res) => res.clone(),
                        None => None
                    }
                } else {
                    None
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

    pub fn has(&'a self, key: &str) -> bool {
        match &self.parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                match col_index.get(key) {
                    Some(idx) => {
                        match self.inner.upgrade().unwrap().get(*idx) {
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

    pub fn to_trio<'b>(&self, buf: &'b mut String) -> fmt::Result {
        match &self.parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                if !cols.is_empty() {
                    let mut iter = cols.iter().enumerate().peekable();
                    while let Some((idx,_c)) = iter.next() {
                        match self.inner.upgrade().unwrap().get(idx) {
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

    pub fn to_zinc<'b>(&self, buf: &'b mut String) -> fmt::Result {
        match &self.parent {
            HGrid::Grid { meta, col_index, cols, rows } => {
                if !cols.is_empty() {
                    let mut iter = cols.iter().enumerate().peekable();
                    while let Some((idx,_c)) = iter.next() {
                        match self.inner.upgrade().unwrap().get(idx) {
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

impl <'a,T: NumTrait>Debug for HRow<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "}})")?;
        Ok(())
    }
}

impl <'a,T: NumTrait>Display for HRow<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "{:?}",self)?;
        write!(f, "}})")?;
        Ok(())
    }
}