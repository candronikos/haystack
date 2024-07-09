use core::slice::SliceIndex;
use core::ops::Index;
use std::slice::Iter;
use num::Float;
use crate::{HVal,HType};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

use std::collections::HashMap;

pub mod h_col;
pub use h_col::{Col,HCol};

pub mod h_row;
pub use h_row::{Row,HRow};

pub struct HGrid<'a,T> {
    meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>,
    col_index: HashMap<String, usize>,
    cols: Vec<Col<'a,T>>,
    rows: Vec<Row<'a,T>>,
}

pub type Grid<'a,T> = HGrid<'a,T>;

#[derive(Debug)]
pub enum HGridErr {
    NotFound,
    IndexErr
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

        let mut grid = HGrid { meta, col_index, cols, rows:Vec::new() };

        let rows = grid_rows.into_iter().map(|r| {
            for (k,_) in r.iter() {
                let col_name = k.to_string();
                if !grid.col_index.contains_key(&col_name) {
                    let len = grid.col_index.len();
                    grid.col_index.insert(col_name,len);
                    grid.cols.push(Col::new(k.to_string(), None));
                }
            }

            let row: Vec<Option<Box<dyn HVal<T>>>> = grid.cols.iter().map(|c| r.remove(c.name.as_str())).collect();
            Row::new(row)
        }).collect();

        grid.rows = rows;
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

        HGrid { meta, col_index, cols, rows }
    }

    pub fn add_meta(mut self, meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>) -> Result<HGrid<'a,T>,HGridErr> {
        self.meta.extend(meta);
        Ok(self)
    }

    pub fn add_col_meta(mut self, col: &str, meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>) -> Result<Self,HGridErr> {
        let idx = self.col_index.get(col).ok_or(HGridErr::NotFound)?;
        self.cols.get_mut(*idx).ok_or(HGridErr::NotFound)?
            .add_meta(meta);
        Ok(self)
    }

    pub fn get(self: &'g Self, key: usize) -> Result<&Row<'a,T>,HGridErr> {
        self.rows.get(key).ok_or(HGridErr::IndexErr)
    }

    pub fn first(self: &'g Self) -> Result<&Row<'a,T>,HGridErr> {
        self.rows.get(0).ok_or(HGridErr::IndexErr)
    }

    pub fn last(self: &'g Self) -> Result<&Row<'a,T>,HGridErr> {
        let length = self.rows.len();
        self.rows.get(length-1).ok_or(HGridErr::IndexErr)
    }

    pub fn has(&self, key: &str) -> bool {
        self.col_index.contains_key(key)
    }

    pub fn meta(&'g self) -> &'g HashMap<String, Box<(dyn HVal<'a, T> + 'a)>> {
        &self.meta
    }

    pub fn iter_cols(&'g self) -> Iter<'_, HCol<'a, T>> {
        self.cols.iter()
    }

    // pub fn iter(&'g self) -> &HRow<'a,T> {
    pub fn iter(&'g self) -> Iter<'_, HRow<'a, T>> {
        // pub fn iter(&self) -> Box<dyn Iterator<Item=&HRow<'_,T>> + '_> {
        // HGridIter { index:0, grid:self }
        self.rows.iter()
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
        write!(buf,"ver:\"3.0\" ")?;
        if !self.meta.is_empty() {
            let mut iter = self.meta.iter().peekable();
            while let Some((k,v)) = iter.next() {
                write!(buf, " {}", k.as_str())?;
                match v.haystack_type() {
                    HType::Marker => (),
                    _ => { write!(buf, ":")?; v.to_zinc(buf)?; }
                };
            }
        }
        write!(buf, "\n")?;
        if !self.cols.is_empty() {
            let mut iter = self.cols.iter().peekable();
            while let Some(c) = iter.next() {
                c.to_zinc(buf)?;
                if let Some(_) = iter.peek() {
                    write!(buf, ", ")?;
                }
            }
        }
        write!(buf, "\n")?;
        if !self.rows.is_empty() {
            let mut iter = self.rows.iter().peekable();
            while let Some(r) = iter.next() {
                r.to_zinc(self, buf)?;
                write!(buf, "\n")?;
            }
        }
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
    use super::*;
    use super::super::{MARKER,REMOVE};

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