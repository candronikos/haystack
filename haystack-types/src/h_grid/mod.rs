use core::{hash, panic};
use core::slice::SliceIndex;
use core::ops::Index;
use std::rc::Rc;
use std::slice::Iter;
use num::Float;
use crate::h_str::HStr;
use crate::io::HBox;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, write, Display, Write};
use std::str::FromStr;

use std::collections::HashMap;

pub mod h_col;
pub use h_col::{Col,HCol};

pub mod h_row;
pub use h_row::{Row,HRow};

pub enum HGrid<'a, T: NumTrait + 'a> {
    Grid {
        meta: HashMap<String, HBox<'a,T>>,
        col_index: HashMap<String, usize>,
        cols: Vec<HCol<'a,T>>,
        rows: Vec<HRow<'a,T>>,
    },
    Error {
        dis: String,
        errTrace: Option<String>,
    },
    Empty { meta: Option<HashMap<String, HBox<'a,T>>> },
}

impl<'a, T: NumTrait + 'a> fmt::Debug for HGrid<'a, T> {
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
    NotImplemented
}

const THIS_TYPE: HType = HType::Grid;

impl <'a,T: NumTrait + 'a>HGrid<'a,T> {
    pub fn new(g_columns: Option<Vec<HCol<'a,T>>>, grid_rows: Vec<HashMap<String, HBox<'a,T>>>) -> HGrid<'a,T> {
        let meta = HashMap::with_capacity(0);
        let mut col_index: HashMap<String, _> = HashMap::new();
        let mut cols = Vec::new();

        if let Some(columns) = g_columns {
            let mut col_iter = columns.iter();
            for c in col_iter.by_ref() {
                let len = col_index.len();
                let c_name = &c.name;
                col_index.insert(c_name.clone(),len);
            }
            cols = columns;
        }

        let rows = grid_rows.into_iter().map(|mut r| {
            for (k,_) in r.iter() {
                let col_name = k.to_string();
                if !col_index.contains_key(&col_name) {
                    let len = col_index.len();
                    col_index.insert(col_name,len);
                    cols.push(Col::new(k.to_string(), None));
                }
            }

            let row: Vec<Option<HBox<'a,T>>> = cols.iter().map(|c| r.remove(c.name.as_str())).collect();
            Row::new(row)
        }).collect();

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
        let mut cols: Vec<HCol<'_, T>> = Vec::new();

        for (name,meta) in columns.into_iter() {
            if !col_index.contains_key(name.as_str()) {
                let len = col_index.len();
                col_index.insert(name.to_owned(),len);
                cols.push(Col::new(name.to_owned(), meta));
            } else {
                panic!("Attempting to read grid with multiple columns of the same name")
            }
        }

        let rows: Vec<HRow<'_, T>> = grid_rows.into_iter().map(|r| {
            Row::new(r)
        }).collect();

        HGrid::Grid{ meta, col_index, cols, rows }
    }

    pub fn add_meta(mut self, meta: HashMap<String, HBox<'a,T>>) -> Result<HGrid<'a,T>,HGridErr> {
        match &mut self {
            HGrid::Grid { meta: orig_meta, .. } => {
                orig_meta.extend(meta);
            },
            HGrid::Error { dis, errTrace } => {
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

    pub fn get(&'a self, key: usize) -> Result<&'a Row<'a,T>,HGridErr> {
        match self {
            HGrid::Grid { rows, .. } => rows.get(key).ok_or(HGridErr::IndexErr),
            _ => Err(HGridErr::IndexErr),
        }
    }

    pub fn first(&'a self) -> Result<&'a Row<'a,T>,HGridErr> {
        match self {
            HGrid::Grid { rows, .. } => rows.get(0).ok_or(HGridErr::IndexErr),
            HGrid::Error { dis, errTrace } => Err(HGridErr::NotImplemented),
            _ => Err(HGridErr::IndexErr),
        }
    }

    pub fn last(&'a self) -> Result<&'a Row<'a,T>,HGridErr> {
        if let HGrid::Grid { rows, .. } = self {
            let length = rows.len();
            rows.get(length - 1).ok_or(HGridErr::IndexErr)
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

    pub fn meta(&'a self) -> &'a HashMap<String, HBox<'a, T>> {
        match self {
            HGrid::Grid { meta, .. } => meta,
            HGrid::Error { dis, errTrace } => todo!("Not implemented"),
            HGrid::Empty { meta } => todo!("Not implemented"),
        }
    }

    pub fn iter_cols(&'a self) -> Iter<'a, HCol<'a, T>> {
        match self {
            HGrid::Grid { cols, .. } => cols.iter(),
            _ => panic!("Cannot iterate columns on non-Grid variant"),
        }
    }

    pub fn iter(&'a self) -> Iter<'a, HRow<'a, T>> {
        match self {
            HGrid::Grid { rows, .. } => rows.iter(),
            HGrid::Empty { .. } => Iter::default(),
            HGrid::Error { dis, errTrace } => {
                panic!("Cannot iterate rows on Error variant: {:?} {:?}", dis, errTrace)
            }
        }
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}

/*
impl <'a,T,I>Index<I> for HGrid<'a,T>
where
    I: SliceIndex<[HRow<'a, T>]>,
{
    type Output = &'a I::Output;
    // type Output = <I as SliceIndex<[HRow<'a,T>]>>::Output;

    fn index(&self, index: I) -> Self::Output {
        &self.rows[index]
    }
}
*/

// impl <'g,'a:'g,T:'a + Float + Display + FromStr>IntoIterator for &'g HGrid<'a,T> {
//     // type Item = &'a HRow<'a,T>;
//     type Item = &'a HRow<'a,T>;
//     type IntoIter = HGridIter<'g,'a,T>;

//     fn into_iter(self) -> Self::IntoIter {
//         HGridIter { index:0, grid:self }
//     }
// }

// pub struct HGridIter<'g,'a,T> {
//     index: usize,
//     grid: &'g HGrid<'a,T>,
// }

// impl <'g,'a:'g,T:'a + Float + Display + FromStr>Iterator for HGridIter<'g,'a,T> {
//     type Item = &'a HRow<'a,T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let grid: &'g HGrid<'a, T> = self.grid;
//         let ret = grid.get(self.index).ok()?;
//         Some(ret)
//     }
// }

impl <'a,T:'a + NumTrait + 'a>HVal<'a,T> for HGrid<'a,T> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        match self {
            HGrid::Grid { meta, col_index, cols, rows } => {
                write!(buf,"ver:\"3.0\" ")?;
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
                write!(buf, "\n")?;
                if !cols.is_empty() {
                    let mut iter = cols.iter().peekable();
                    while let Some(c) = iter.next() {
                        c.to_zinc(buf)?;
                        if let Some(_) = iter.peek() {
                            write!(buf, ", ")?;
                        }
                    }
                }
                write!(buf, "\n")?;
                if !rows.is_empty() {
                    let mut iter = rows.iter().peekable();
                    while let Some(r) = iter.next() {
                        r.to_zinc(self, buf)?;
                        write!(buf, "\n")?;
                    }
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

                write!(buf,"empty\n")
            }
        }
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        match self {
            HGrid::Grid { meta, col_index, cols, rows } => {
                let mut row_iter = rows.iter().peekable();
                
                while let Some(row) = row_iter.next() {
                    row.to_trio(self, buf)?;
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
    set_get_method!(get_grid_val, HGrid<'a,T>);
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
        grid_meta.insert("meta1".into(), Rc::new(MARKER));
        grid_meta.insert("meta2".into(), Rc::new(REMOVE));

        let mut col_meta: HashMap<String,HBox<f64>> = HashMap::new();
        col_meta.insert("cmeta1".into(), Rc::new(MARKER));
        col_meta.insert("cmeta2".into(), Rc::new(REMOVE));
        col_meta.insert("cmeta3".into(), Rc::new(MARKER));

        let mut row_1: HashMap<String,HBox<f64>> = HashMap::new();
        row_1.insert("col1".into(), Rc::new(MARKER));
        row_1.insert("col2".into(), Rc::new(MARKER));
        

        let mut row_2: HashMap<String,HBox<f64>> = HashMap::new();
        row_2.insert("col1".into(), Rc::new(REMOVE));
        row_2.insert("col3".into(), Rc::new(REMOVE));

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