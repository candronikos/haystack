use crate::{HVal};
use std::fmt::{self,Write,Display};
use crate::h_grid::Grid;

use std::collections::HashMap;

pub struct HRow<'a> {
    inner: HashMap<&'a str, Box<dyn HVal>>
}

pub type Row<'a> = HRow<'a>;

impl <'a>HRow<'a> {
    pub fn new(inner: HashMap<&'a str, Box<dyn HVal>>) -> Self {
        Self { inner }
    }

    pub fn get(&self, key: &'a str) -> Option<&Box<dyn HVal>> {
        self.inner.get(key)
    }

    pub fn has(&self, key: &'a str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn to_zinc(&self, buf: &mut String, grid: &Grid) ->  fmt::Result {
        if !grid.cols.is_empty() {
            let mut iter = grid.cols.iter().peekable();
            while let Some(c) = iter.next() {
                match self.inner.get(c.name()) {
                    Some(v) => v.to_zinc(buf),
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

impl <'a>Display for HRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HRow({{")?;
        write!(f, "}})")?;
        Ok(())
    }
}