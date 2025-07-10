use crate::h_val::HBox;
use crate::{HCast, HType, NumTrait};
use std::fmt::{self, Write};

use std::collections::HashMap;

#[derive(Clone)]
pub struct HCol<'a, T: NumTrait> {
    pub name: String,
    meta: HashMap<String, HBox<'a, T>>,
}

impl<'a, T: NumTrait> fmt::Debug for HCol<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HCol")
            .field("name", &self.name)
            .field(
                "meta",
                &format_args!("{:?}", self.meta.keys().collect::<Vec<_>>()),
            )
            .finish()
    }
}

pub type Col<'a, T> = HCol<'a, T>;

impl<'a, T: NumTrait> HCol<'a, T> {
    pub fn new(name: String, meta: Option<HashMap<String, HBox<'a, T>>>) -> Self {
        Self {
            name,
            meta: meta.unwrap_or(HashMap::new()),
        }
    }
}

impl<'a, T: NumTrait> HCol<'a, T> {
    // pub fn name<'a>(&'a self) -> &'a str {
    //     &self.name
    // }

    pub fn get(&self, key: String) -> Option<&HBox<'a, T>> {
        self.meta.get(&key)
    }

    pub fn has(&self, key: String) -> bool {
        self.meta.contains_key(&key)
    }

    pub fn add_meta(&mut self, meta: HashMap<String, HBox<'a, T>>) {
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
            while let Some((k, v)) = iter.next() {
                write!(buf, "{}", k)?;
                match v.haystack_type() {
                    HType::Marker => (),
                    _ => {
                        write!(buf, ":")?;
                        v.to_zinc(buf)?;
                    }
                };
                if let Some(_) = iter.peek() {
                    write!(buf, " ")?;
                }
            }
        }
        Ok(())
    }
}
