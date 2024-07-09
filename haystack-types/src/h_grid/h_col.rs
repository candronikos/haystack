use num::Float;
use crate::{HCast,HVal,HType};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

use std::collections::HashMap;

pub struct HCol<'a,T> {
    pub name: String,
    meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>
}

pub type Col<'a,T> = HCol<'a,T>;

impl <'a,T:'a + Float + Display + FromStr>HCol<'a,T> {
    pub fn new(name: String, meta: Option<HashMap<String, Box<dyn HVal<'a,T> + 'a>>>) -> Self {
        Self {
            name,
            meta: meta.unwrap_or(HashMap::new())
        }
    }
}

impl <'a,T:'a + Float + Display + FromStr>HCol<'a,T> {
    // pub fn name<'a>(&'a self) -> &'a str {
    //     &self.name
    // }

    pub fn get(&self, key: String) -> Option<&Box<dyn HVal<'a,T> + 'a>> {
        self.meta.get(&key)
    }

    pub fn has(&self, key: String) -> bool {
        self.meta.contains_key(&key)
    }

    pub fn add_meta(&mut self, meta: HashMap<String, Box<dyn HVal<'a,T> + 'a>>) {
        self.meta.extend(meta)
    }

    pub fn dis(&self) -> String {
        let meta = &self.meta;
        if let Some(s) = meta.get("dis") {
            s.get_string().unwrap().as_str().to_owned()
        } else if let Some(s) = meta.get("disMacro") {
            todo!()
        } else if let Some(s) = meta.get("disKey") {
            todo!()
        } else if let Some(s) = meta.get("name") {
            s.get_string().unwrap().as_str().to_owned()
        } else if let Some(s) = meta.get("tag") {
            s.get_string().unwrap().as_str().to_owned()
        } else if let Some(s) = meta.get("id") {
            todo!()
        } else {
            "!default".to_owned()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
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