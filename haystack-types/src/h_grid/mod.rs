use core::{hash, panic};
use crate::h_dict::HDict;
use crate::h_str::HStr;
use crate::h_val::HBox;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

use std::collections::HashMap;
use rpds::Vector;

pub mod h_col;
pub use h_col::{Col,HCol};

pub mod h_row;
pub use h_row::{Row,HRow};

use std::rc::Rc;
use std::cell::{Ref, RefCell};

#[derive(Clone)]
pub enum HGrid<'a, T: NumTrait + 'a>{
    Grid {
        meta: RefCell<HDict<'a,T>>,
        col_index: Rc<HashMap<String, usize>>,
        cols: Vector<HCol<'a,T>>,
        rows: Vec<Rc<Vector<Option<HBox<'a,T>>>>>,
    },
    Error {
        dis: String,
        errTrace: Option<String>,
    },
    Empty { meta: Option<HashMap<String, HBox<'a,T>>> },
}

impl<'a, T: NumTrait> fmt::Debug for HGrid<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HGrid::Grid { .. } => {
                write!(f, "HGrid::Grid {{}}")
            }
            HGrid::Error { dis, errTrace } => {
                f.debug_struct("HGrid::Error")
                    .field("dis", dis)
                    .field("errTrace", errTrace)
                    .finish()
            }
            HGrid::Empty { .. } => write!(f, "HGrid::Empty"),
        }
    }
}

pub type Grid<'a,T> = HGrid<'a,T>;

#[derive(Debug)]
pub enum HGridErr {
    NotFound,
    IndexErr,
    NotImplemented,
    AddMetaFailed,
}

impl fmt::Display for HGridErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HGridErr::NotFound => write!(f, "Error: Not Found"),
            HGridErr::IndexErr => write!(f, "Error: Index Out of Bounds"),
            HGridErr::NotImplemented => write!(f, "Error: Not Implemented"),
            HGridErr::AddMetaFailed => write!(f, "Error: Failed to add grid metadata"),
        }
    }
}

const THIS_TYPE: HType = HType::Grid;

impl <'a,T: NumTrait + 'a>HGrid<'a,T> {
    pub fn new(g_columns: Option<Vec<HCol<'a,T>>>, grid_rows: Vec<HashMap<String, HBox<'a,T>>>) -> HGrid<'a,T> {
        let meta = HashMap::with_capacity(0);
        let mut col_index: HashMap<String, _> = HashMap::new();
        let mut cols = Vector::new();

        if let Some(columns) = g_columns {
            let mut col_iter = columns.iter();
            for c in col_iter.by_ref() {
                let len = col_index.len();
                let c_name = &c.name;
                col_index.insert(c_name.clone(),len);
                cols.push_back_mut(c.clone());
            }
        }

        let rows = grid_rows.into_iter().map(|mut r| {
            for (k,_) in r.iter() {
                let col_name = k.to_string();
                if !col_index.contains_key(&col_name) {
                    let len = col_index.len();
                    col_index.insert(col_name,len);
                    cols.push_back_mut(Col::new(k.to_string(), None));
                }
            }

            let row: Vector<Option<HBox<'a,T>>> = cols.iter().map(|c| r.remove(c.name.as_str())).collect();
            Rc::from(row)
        }).collect();

        let meta = RefCell::new(HDict::from_map(meta));
        let col_index = Rc::new(col_index);

        let grid = HGrid::Grid {
            meta,
            col_index,
            cols,
            rows
        };

        grid
    }

    pub fn from_row_vec<'b>(columns: Vec<(String,Option<HashMap<String,HBox<'b,T>>>)>, grid_rows: Vec<Vec<Option<HBox<'b,T>>>>) -> Grid<'b,T> {
        let meta = HashMap::with_capacity(0);
        let mut col_index: HashMap<String, _> = HashMap::new();
        let mut cols: Vector<HCol<'_, T>> = Vector::new();

        for (name,meta) in columns.into_iter() {
            if !col_index.contains_key(name.as_str()) {
                let len = col_index.len();
                col_index.insert(name.to_owned(),len);
                cols.push_back_mut(Col::new(name.to_owned(), meta));
            } else {
                panic!("Attempting to read grid with multiple columns of the same name")
            }
        }

        let rows = grid_rows.into_iter().map(|r| {
            let mut v = Vector::new();
            
            r.into_iter().for_each(|val| {
                v = v.push_back(val)
            });

            Rc::from(v)
        }).collect();

        let meta = RefCell::new(HDict::from_map(meta));
        let col_index = Rc::new(col_index);

        HGrid::Grid{ meta, col_index, cols, rows }
    }

    pub fn add_meta(mut self, meta: HashMap<String, HBox<'a,T>>) -> Result<HGrid<'a,T>,HGridErr> {
        match &mut self {
            HGrid::Grid { meta: orig_meta, .. } => {
                orig_meta.borrow_mut().extend(meta);
            },
            HGrid::Error { .. } => {
                return Err(HGridErr::NotImplemented);
            },
            HGrid::Empty { meta: inner } => {
                match inner {
                    Some(inner_meta) => inner_meta.extend(meta),
                    None => _ = inner.replace(meta),
                };
            }
        }
        Ok(self)
    }

    pub fn add_col_meta(mut self, col: &str, meta: HashMap<String, HBox<'a,T>>) -> Result<Self,HGridErr> {
        match &mut self {
            HGrid::Grid { col_index, cols, .. } => {
                let idx = col_index.get(col).ok_or(HGridErr::NotFound)?;
                cols.get_mut(*idx).ok_or(HGridErr::NotFound)?
                    .add_meta(meta);
            },
            HGrid::Error { dis, errTrace } => {
                return Err(HGridErr::NotImplemented);
            },
            HGrid::Empty { .. } => {
                return Err(HGridErr::NotFound);
            }
        }
        Ok(self)
    }

    pub fn get(&self, key: usize) -> Result<HRow<'a,T>,HGridErr> {
        match self {
            HGrid::Grid { meta, cols, col_index, rows, .. } => {
                let r = rows.get(key).ok_or(HGridErr::IndexErr)?;
                let parent = HGrid::Grid {
                    meta: RefCell::new(meta.borrow().clone()),
                    col_index: Rc::clone(col_index),
                    cols: cols.clone(),
                    rows: vec![],
                };
                Ok(HRow::new(parent, Rc::downgrade(r)))
            },
            _ => Err(HGridErr::IndexErr),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            HGrid::Grid { rows, .. } => rows.len(),
            _ => 0
        }
    }

    pub fn first(&self) -> Result<HRow<'a,T>,HGridErr> {
        match self {
            HGrid::Grid { .. } => self.get(0),
            HGrid::Error { dis, errTrace } => Err(HGridErr::NotImplemented),
            _ => Err(HGridErr::IndexErr),
        }
    }

    pub fn last(&self) -> Result<HRow<'a,T>,HGridErr> {
        if let HGrid::Grid { .. } = self {
            let length = self.len();
            self.get(length - 1)
        } else {
            Err(HGridErr::IndexErr)
        }
    }

    pub fn has(&self, key: &str) -> bool {
        match self {
            HGrid::Grid { col_index, .. } => col_index.contains_key(key),
            HGrid::Error { .. } => key=="err" || key=="errTrace" || key=="dis",
            _ => false,
        }
    }

    pub fn meta(&self) -> HDict<'a, T> {
        match self {
            HGrid::Grid { meta, .. } => meta.borrow().clone(),
            HGrid::Error { dis, errTrace } => todo!("Not implemented"),
            HGrid::Empty { meta } => todo!("Not implemented"),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            HGrid::Grid { rows, .. } => rows.is_empty(),
            _ => true,
        }
    }

    /*
    pub fn iter_cols(&self) -> impl Iterator<Item = &HCol<'a, T>> + '_ {
        match self {
            HGrid::Grid { cols, .. } => cols.borrow().iter(),
            _ => [].iter(),
        }
    }
    */
    
    pub fn iter_cols(&self) -> HColIter<'a, T> {
        match self {
            HGrid::Grid { cols, .. } => HColIter {
                cols: Some(cols.clone()),
                index: 0,
            },
            _ => HColIter {
                cols: None,
                index: 0,
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = HRow<'a, T>>
    where
        T: Clone + 'a
    {
            match self {
            HGrid::Grid { rows, meta, col_index, cols } => {
                let parent = HGrid::Grid {
                    meta:meta.clone(),
                    col_index:col_index.clone(),
                    cols:cols.clone(),
                    rows: vec![] };
                let static_rows = rows
                    .iter()
                    .map(|row| HRow::new(parent.clone(),(Rc::<Vector<Option<Rc<(dyn HVal<'a, T>)>>>>::downgrade(row))))
                    .collect::<Vec<HRow<'a,T>>>()
                    .into_iter();
                static_rows
            },
            HGrid::Empty { .. } => panic!("Empty grid"),//Box::new(std::iter::empty()),
            HGrid::Error { dis, errTrace } => {
                panic!("Cannot iterate rows on Error variant: {:?} {:?}", dis, errTrace)
            }
        }
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}

pub struct HColIter<'a, T: NumTrait + 'a> {
    cols: Option<Vector<HCol<'a, T>>>,
    index: usize,
}

impl<'a, T: NumTrait + 'a> Iterator for HColIter<'a, T> {
    type Item = HCol<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref cols) = self.cols {
            if self.index < cols.len() {
                let col = cols[self.index].clone();
                self.index += 1;
                Some(col)
            } else {
                None
            }
        } else {
            None
        }
    }
}

// impl <'a,T: NumTrait + 'a,I>Index<I> for HGrid<'a,T>
// where
//     I: SliceIndex<[HRow<'a, T>]>,
// {
//     type Output = &'a I::Output;
//     // type Output = <I as SliceIndex<[HRow<'a,T>]>>::Output;

//     fn index(&self, index: I) -> Self::Output {
//         &self.rows[index]
//     }
// }

impl <'a,T: NumTrait + 'a>IntoIterator for &'a HGrid<'a,T> {
    type Item = HRow<'a,T>;
    type IntoIter = HGridIter<'a,T>;

    fn into_iter(self) -> Self::IntoIter {
        HGridIter { index:0, grid:self }
    }
}

impl <'a,T: NumTrait + 'a>IntoIterator for &'a mut HGrid<'a,T> {
    type Item = HRow<'a,T>;
    type IntoIter = HGridIter<'a,T>;

    fn into_iter(self) -> Self::IntoIter {
        HGridIter { index:0, grid:self }
    }
}

pub struct HGridIter<'a, T: NumTrait + 'a> {
    index: usize,
    grid: &'a HGrid<'a,T>,
}

impl <'a,T: NumTrait + 'a>HGridIter<'a,T> {
    pub fn new(grid: &'a HGrid<'a,T>) -> Self {
        HGridIter { index: 0, grid }
    }
}

impl <'a,T:NumTrait + 'a>Iterator for HGridIter<'a,T> {
    type Item = HRow<'a,T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.len() {
            return None;
        }
        let ret = self.grid.get(self.index).ok()?;
        self.index += 1;
        Some(ret)
    }
}

impl <'a,T:'a + NumTrait>HVal<'a,T> for HGrid<'a,T> {
    fn to_zinc<'b>(&self, buf: &'b mut String) -> fmt::Result {
        match self {
            HGrid::Grid { meta, rows, .. } => {
                write!(buf,"ver:\"3.0\" ")?;
                if !meta.borrow().is_empty() {
                    let meta_borrow = meta.borrow();
                    let mut iter = meta_borrow.iter().peekable();
                    while let Some((k,v)) = iter.next() {
                        write!(buf, " {}", k.as_str())?;
                        match v.haystack_type() {
                            HType::Marker => (),
                            _ => { write!(buf, ":")?; v.to_zinc(buf)?; }
                        };
                    }
                }
                write!(buf, "\n")?;

                let mut iter = self.iter_cols().peekable();
                while let Some(c) = iter.next() {
                    let () = c.to_zinc(buf)?;
                    if let Some(_) = iter.peek() {
                        write!(buf, ", ")?;
                    }
                }
                
                write!(buf, "\n")?;

                let mut iter = self.iter();
                while let Some(r) = iter.next() {
                    r.to_zinc(buf)?;
                    write!(buf, "\n")?;
                }
                Ok(())
            },
            HGrid::Error { dis, errTrace } => {
                //write!(buf,"ver:\"3.0\" err dis:{} errTrace:{}\nempty",dis,HStr(errTrace.toString))?;
                write!(buf,"ver:\"3.0\" err dis:")?;
                HVal::<f64>::to_zinc(&HStr(dis.to_string()), buf)?;

                if let Some(errTrace) = errTrace {
                    write!(buf," errTrace:")?;
                    HVal::<f64>::to_zinc(&HStr(errTrace.to_string()), buf)?;
                }
                write!(buf,"\nempty\n")
            },
            HGrid::Empty { meta } => {
                write!(buf,"ver:\"3.0\"")?;
                
                if let Some(meta) = meta {
                    if !meta.is_empty() {
                        let mut iter = meta.iter().peekable();
                        while let Some((k,v)) = iter.next() {
                            write!(buf, " {}", k.as_str())?;
                            match v.haystack_type() {
                                HType::Marker => (),
                                _ => { write!(buf, ":")?; v.to_zinc(buf)?; }
                            };
                        }
                    }
                }

                write!(buf,"\nempty\n")
            }
        }
    }
    fn to_trio<'b>(&self, buf: &'b mut String) -> fmt::Result {
        match self {
            HGrid::Grid { .. } => {
                let mut row_iter = self.iter().peekable();
                
                while let Some(row) = row_iter.next() {
                    row.to_trio(buf)?;
                    if row_iter.peek().is_some() {
                        write!(buf, "---\n")?;
                    }
                }
            },
            _ => ()
        };
        Ok(())
    }
    fn to_json(&self, _buf: &mut String) -> fmt::Result {
        unimplemented!();
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    fn _eq(&self, other: &dyn HVal<'a,T>) -> bool { false }
    set_get_method!(get_grid_val,HGrid,'a,T);
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use super::super::{MARKER,REMOVE};

    const EMPTY_GRID: &str = "ver:\"3.0\"\nempty\n";
    const ERROR_GRID: &str = "ver:\"3.0\" err dis:\"Display message\"\nempty\n";
    const ERROR_GRID_TRACE: &str = "ver:\"3.0\" err dis:\"Display message\" errTrace:\"Error trace (Optional)\"\nempty\n";

    #[test]
    fn print_grid() {
        let mut grid_meta: HashMap<String,HBox<f64>> = HashMap::new();
        grid_meta.insert("meta1".into(), MARKER.to_hbox());
        grid_meta.insert("meta2".into(), REMOVE.to_hbox());

        let mut col_meta: HashMap<String,HBox<f64>> = HashMap::new();
        col_meta.insert("cmeta1".into(), MARKER.to_hbox());
        col_meta.insert("cmeta2".into(), REMOVE.to_hbox());
        col_meta.insert("cmeta3".into(), MARKER.to_hbox());

        let mut row_1: HashMap<String,HBox<f64>> = HashMap::new();
        row_1.insert("col1".into(), MARKER.to_hbox());
        row_1.insert("col2".into(), MARKER.to_hbox());
        

        let mut row_2: HashMap<String,HBox<f64>> = HashMap::new();
        row_2.insert("col1".into(), REMOVE.to_hbox());
        row_2.insert("col3".into(), REMOVE.to_hbox());

        let mut grid = Grid::new(None,vec![row_1,row_2]);
        grid = grid.add_meta(grid_meta).unwrap();
        grid = grid.add_col_meta("col1",col_meta).unwrap();

        let mut buf = String::new();
        {
            grid.to_zinc(&mut buf).unwrap();
        }
        println!("GRID\n{}", buf);
    }
}

// TODO: Implement serialisation tests for HGrid