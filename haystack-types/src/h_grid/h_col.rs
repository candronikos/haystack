use crate::{HVal,HType};
use std::fmt::{self,Write};

use std::collections::HashMap;

#[derive(Debug)]
pub struct HCol {
    pub name: String,
    meta: HashMap<String, Box<dyn HVal>>
}

pub type Col = HCol;

impl HCol {
    pub fn new(name: String, meta: Option<HashMap<String, Box<dyn HVal>>>) -> Self {
        Self {
            name,
            meta: meta.unwrap_or(HashMap::new())
        }
    }
}

impl HCol {
    // pub fn name<'a>(&'a self) -> &'a str {
    //     &self.name
    // }

    pub fn get(&self, key: String) -> Option<&Box<dyn HVal>> {
        self.meta.get(&key)
    }

    pub fn has(&self, key: String) -> bool {
        self.meta.contains_key(&key)
    }

    pub fn add_meta(&mut self, meta: HashMap<String, Box<dyn HVal>>) {
        self.meta.extend(meta)
    }

    pub fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "{}", self.name)?;

        if !self.meta.is_empty() {
            write!(buf, " ")?;
            let mut iter = self.meta.iter().peekable();
            while let Some((k,v)) = iter.next() {
                write!(buf, "{}", k)?;
                match v.haystack_type() {
                    HType::Marker => (),
                    _ => { write!(buf, ":")?; v.to_zinc(buf)?; }
                };
                if let Some(_) = iter.peek() {
                    write!(buf, " ")?;
                }
            }
        }
        Ok(())
    }
}