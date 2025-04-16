use core::{hash, panic};
use core::slice::SliceIndex;
use core::ops::Index;
use std::slice::Iter;
use num::Float;
use crate::h_str::HStr;
use crate::{HVal,HType};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

use std::collections::HashMap;

pub mod h_col;
pub use h_col::{Col,HCol};

pub mod h_row;
pub use h_row::{Row,HRow};

pub enum HGrid<'a, T> {
    Grid {
        meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>,
        col_index: HashMap<String, usize>,
        cols: Vec<HCol<'a,T>>,
        rows: Vec<HRow<'a,T>>,
    },
    Error {
        dis: String,
        errTrace: Option<String>,
    },
    Empty,
}

pub type Grid<'a,T> = HGrid<'a,T>;

#[derive(Debug)]
pub enum HGridErr {
    NotFound,
    IndexErr,
    NotImplemented
}

const THIS_TYPE: HType = HType::Grid;

impl <'g,'a:'g,T:'a + Float + Display + FromStr>HGrid<'a,T> {
    pub fn new(g_columns: Option<Vec<HCol<'a,T>>>, grid_rows: &mut Vec<HashMap<&str, Box<dyn HVal<'a,T>>>>) -> Grid<'a,T> {
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

        let rows: Vec<HRow<'a, T>> = grid_rows.into_iter().map(|r| {
            for (k,_) in r.iter() {
                let col_name = k.to_string();
                if !col_index.contains_key(&col_name) {
                    let len = col_index.len();
                    col_index.insert(col_name,len);
                    cols.push(Col::new(k.to_string(), None));
                }
            }

            let row: Vec<Option<Box<dyn HVal<T>>>> = cols.iter().map(|c| r.remove(c.name.as_str())).collect();
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

    pub fn from_row_vec(columns: Vec<(String,Option<HashMap<String,Box<dyn HVal<'a,T> + 'a>>>)>, mut grid_rows: Vec<Vec<Option<Box<dyn HVal<'a,T> + 'a>>>>) -> Grid<'a,T> {
        let meta = HashMap::with_capacity(0);
        let mut col_index: HashMap<String, _> = HashMap::new();
        let mut cols = Vec::new();

        for (name,meta) in columns.into_iter() {
            if !col_index.contains_key(name.as_str()) {
                let len = col_index.len();
                col_index.insert(name.to_owned(),len);
                cols.push(Col::new(name.to_owned(), meta));
            } else {
                panic!("Attempting to read grid with multiple columns of the same name")
            }
        }

        let rows = grid_rows.into_iter().map(|r| {
            Row::new(r)
        }).collect();

        HGrid::Grid{ meta, col_index, cols, rows }
    }

    pub fn add_meta(mut self, meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>) -> Result<HGrid<'a,T>,HGridErr> {
        match &mut self {
            HGrid::Grid { meta: orig_meta, .. } => {
                orig_meta.extend(meta);
            },
            HGrid::Error { dis, errTrace } => {
                return Err(HGridErr::NotImplemented);
            },
            HGrid::Empty => {
                self = HGrid::Grid {
                    meta: meta,
                    col_index: HashMap::new(),
                    cols: Vec::new(),
                    rows: Vec::new()
                };
            }
        }
        Ok(self)
    }

    pub fn add_col_meta(mut self, col: &str, meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>) -> Result<Self,HGridErr> {
        match &mut self {
            HGrid::Grid { col_index, cols, .. } => {
                let idx = col_index.get(col).ok_or(HGridErr::NotFound)?;
                cols.get_mut(*idx).ok_or(HGridErr::NotFound)?
                    .add_meta(meta);
            },
            HGrid::Error { dis, errTrace } => {
                return Err(HGridErr::NotImplemented);
            },
            HGrid::Empty => {
                return Err(HGridErr::NotFound);
            }
        }
        Ok(self)
    }

    pub fn get(self: &'g Self, key: usize) -> Result<&Row<'a,T>,HGridErr> {
        match self {
            HGrid::Grid { rows, .. } => rows.get(key).ok_or(HGridErr::IndexErr),
            _ => Err(HGridErr::IndexErr),
        }
    }

    pub fn first(self: &'g Self) -> Result<&Row<'a,T>,HGridErr> {
        match self {
            HGrid::Grid { rows, .. } => rows.get(0).ok_or(HGridErr::IndexErr),
            HGrid::Error { dis, errTrace } => Err(HGridErr::NotImplemented),
            _ => Err(HGridErr::IndexErr),
        }
    }

    pub fn last(self: &'g Self) -> Result<&Row<'a,T>,HGridErr> {
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

    pub fn meta(&'g self) -> &'g HashMap<String, Box<(dyn HVal<'a, T> + 'a)>> {
        match self {
            HGrid::Grid { meta, .. } => meta,
            HGrid::Error { dis, errTrace } => todo!("Not implemented"),
            HGrid::Empty => todo!("Not implemented"),
        }
    }

    pub fn iter_cols(&'g self) -> Iter<'_, HCol<'a, T>> {
        match self {
            HGrid::Grid { cols, .. } => cols.iter(),
            _ => panic!("Cannot iterate columns on non-Grid variant"),
        }
    }

    pub fn iter(&'g self) -> Iter<'_, HRow<'a, T>> {
        match self {
            HGrid::Grid { rows, .. } => rows.iter(),
            HGrid::Empty => Iter::default(),
            HGrid::Error { dis, errTrace } => {
                panic!("Cannot iterate rows on Error variant: {:?} {:?}", dis, errTrace)
            }
        }
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

impl <'a,T:'a + Float + Display + FromStr>HVal<'a,T> for HGrid<'a,T> {
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
                //HStr(dis.to_string()).to_zinc(buf)?;
                <HStr as HVal<'_, T>>::to_zinc(&HStr(dis.to_string()), buf)?;

                if let Some(errTrace) = errTrace {
                    write!(buf," errTrace:")?;
                    //HStr(errTrace.to_string()).to_zinc(buf)?;
                    <HStr as HVal<'_, T>>::to_zinc(&HStr(errTrace.to_string()), buf)?;
                }
                write!(buf,"\nempty\n")
            },
            HGrid::Empty => {
                write!(buf,"ver:\"3.0\"\nempty\n")
            }
        }
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
    use super::*;
    use super::super::{MARKER,REMOVE};

    const EMPTY_GRID: &str = "ver:\"3.0\"\nempty\n";
    const ERROR_GRID: &str = "ver:\"3.0\" err dis:\"Display message\"\nempty\n";
    const ERROR_GRID_TRACE: &str = "ver:\"3.0\" err dis:\"Display message\" errTrace:\"Error trace (Optional)\"\nempty\n";

    #[test]
    fn print_grid() {
        let mut grid_meta: HashMap<String,Box<dyn HVal<f64>>> = HashMap::new();
        grid_meta.insert("meta1".to_string(), Box::new(MARKER));
        grid_meta.insert("meta2".to_string(), Box::new(REMOVE));

        let mut col_meta: HashMap<String,Box<dyn HVal<f64>>> = HashMap::new();
        col_meta.insert("cmeta1".to_string(), Box::new(MARKER));
        col_meta.insert("cmeta2".to_string(), Box::new(REMOVE));
        col_meta.insert("cmeta3".to_string(), Box::new(MARKER));

        let mut row_1: HashMap<&str,Box<dyn HVal<f64>>> = HashMap::new();
        row_1.insert("col1", Box::new(MARKER));
        row_1.insert("col2", Box::new(MARKER));

        let mut row_2: HashMap<&str,Box<dyn HVal<f64>>> = HashMap::new();
        row_2.insert("col1", Box::new(REMOVE));
        row_2.insert("col3", Box::new(REMOVE));

        let grid = Grid::new(None,&mut vec![row_1,row_2])
            .add_meta(grid_meta).unwrap()
            .add_col_meta("col1",col_meta).unwrap();

        let mut buf = String::new();

        grid.to_zinc(&mut buf).unwrap();
        println!("GRID\n{}",buf)
    }
}