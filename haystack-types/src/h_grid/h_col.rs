use crate::{HVal,HType};
use std::fmt::{self,Write};

use std::collections::HashMap;
pub struct HCol<'a> {
    name: &'a str,
    meta: HashMap<&'a str, Box<dyn HVal>>
}

pub type Col<'a> = HCol<'a>;

impl <'a>HCol<'a> {
    pub fn new(name: &'a str, meta: Option<HashMap<&'a str, Box<dyn HVal>>>) -> Self {
        Self {
            name,
            meta: meta.unwrap_or(HashMap::new())
        }
    }
}

impl <'a>HCol<'a> {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn get(&self, key: &'a str) -> Option<&Box<dyn HVal>> {
        self.meta.get(key)
    }

    pub fn has(&self, key: &'a str) -> bool {
        self.meta.contains_key(key)
    }

    pub fn add_meta(&mut self, meta: HashMap<&'a str, Box<dyn HVal>>) {
        self.meta.extend(meta)
    }

    pub fn to_zinc(&self, buf: &mut String) ->  fmt::Result {
        write!(buf, "{}", self.name())?;

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