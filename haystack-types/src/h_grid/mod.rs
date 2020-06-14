use crate::{HVal,HType};
use std::fmt::{self,Write};

use std::collections::HashMap;

mod h_col;
use h_col::Col;

mod h_row;
use h_row::Row;

pub struct HGrid<'a> {
    meta: HashMap<&'a str, Box<dyn HVal>>,
    col_index: HashMap<&'a str, usize>,
    cols: Vec<Col<'a>>,
    rows: Vec<Row<'a>>,
}

pub type Grid<'a> = HGrid<'a>;

#[derive(Debug)]
pub enum HGridErr {
    NotFound,
    IndexErr
}

const THIS_TYPE: HType = HType::Grid;

impl <'a>HGrid<'a> {
    pub fn new(grid_rows: &mut Vec<HashMap<&'a str, Box<dyn HVal>>>) -> Grid<'a> {
        let meta = HashMap::with_capacity(0);
        let mut col_index = HashMap::new();
        let mut cols = Vec::new();

        let rows = grid_rows.drain(..).map(|r| {
            for (k,_) in r.iter() {
                if !col_index.contains_key(k) {
                    let len = col_index.len();
                    col_index.insert(*k,len);
                    cols.push(Col::new(k, None));
                }
            }

            Row::new(r)
        }).collect();

        Self { meta, col_index, cols, rows }
    }

    pub fn add_meta(&mut self, meta: HashMap<&'a str, Box<dyn HVal>>) -> &mut Self {
        self.meta.extend(meta);
        self
    }

    pub fn add_col_meta(&mut self, col: &str, meta: HashMap<&'a str, Box<dyn HVal>>) -> Result<&mut Self,HGridErr> {
        let idx = self.col_index.get(col).ok_or(HGridErr::NotFound)?;
        self.cols.get_mut(*idx).ok_or(HGridErr::NotFound)?
            .add_meta(meta);
        Ok(self)
    }

    pub fn get(&self, key: usize) -> Result<&Row,HGridErr> {
        self.rows.get(key).ok_or(HGridErr::IndexErr)
    }

    pub fn has(&self, key: &'a str) -> bool {
        self.col_index.contains_key(key)
    }
}

impl <'a>HVal for HGrid<'a> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        buf.push_str("ver:\"3.0\"");
        if !self.meta.is_empty() {
            let mut iter = self.meta.iter().peekable();
            while let Some((k,v)) = iter.next() {
                write!(buf, " {}", k)?;
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
                r.to_zinc(buf,self)?;
                write!(buf, "\n")?;
            }
        }
        Ok(())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        unimplemented!();
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{MARKER,REMOVE};

    #[test]
    fn print_grid() {
        let mut grid_meta: HashMap<&str,Box<dyn HVal>> = HashMap::new();
        grid_meta.insert("meta1", Box::new(MARKER));
        grid_meta.insert("meta2", Box::new(REMOVE));

        let mut col_meta: HashMap<&str,Box<dyn HVal>> = HashMap::new();
        col_meta.insert("cmeta1", Box::new(MARKER));
        col_meta.insert("cmeta2", Box::new(REMOVE));
        col_meta.insert("cmeta3", Box::new(MARKER));

        let mut row_1: HashMap<&str,Box<dyn HVal>> = HashMap::new();
        row_1.insert("col1", Box::new(MARKER));
        row_1.insert("col2", Box::new(MARKER));

        let mut row_2: HashMap<&str,Box<dyn HVal>> = HashMap::new();
        row_2.insert("col1", Box::new(REMOVE));
        row_2.insert("col3", Box::new(REMOVE));

        let mut grid = Grid::new(&mut vec![row_1,row_2]);
        grid.add_meta(grid_meta)
            .add_col_meta("col1",col_meta).unwrap();

        let mut buf = String::new();
        grid.to_zinc(&mut buf).unwrap();
        println!("{}",buf); // TODO: IMPLEMENT TEST WITH PartialEq and test structures instead
    }
}