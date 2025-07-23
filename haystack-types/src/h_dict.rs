use crate::io::write::{JsonWriter, ZincWriter};
use crate::{HType, HVal, NumTrait, h_val::HBox};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct HDict<'a, T: NumTrait> {
    inner: HashMap<String, HBox<'a, T>>,
}

pub type Dict<'a, T> = HDict<'a, T>;

const THIS_TYPE: HType = HType::Dict;

impl<'a, T: NumTrait> HDict<'a, T> {
    pub fn new() -> HDict<'a, T> {
        HDict {
            inner: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<String, HBox<'a, T>>) -> HDict<'a, T> {
        HDict { inner: map }
    }

    pub fn has(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn set(&mut self, key: String, value: HBox<'a, T>) -> Option<HBox<'a, T>> {
        self.inner.insert(key, value)
    }

    pub fn merge(&mut self, other: HDict<'a, T>) {
        self.inner.extend(other.inner);
    }

    pub fn extend(&mut self, other: HashMap<String, HBox<'a, T>>) {
        self.inner.extend(other);
    }

    pub fn get(&self, key: &str) -> Option<&HBox<'a, T>> {
        self.inner.get(key)
    }

    pub fn into_map(self) -> HashMap<String, HBox<'a, T>> {
        self.inner
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &HBox<'a, T>)> {
        self.inner.iter()
    }

    pub fn to_zinc<'b>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let inner = &self.inner;
        let mut kv_pairs = inner
            .into_iter()
            .filter(|(_, v)| v.get_null().is_none())
            .peekable();
        while let Some((k, v)) = kv_pairs.next() {
            match v.haystack_type() {
                HType::Remove => write!(f, "-{}", k),
                HType::Marker => write!(f, "{}", k),
                _ => {
                    write!(f, "{}:{}", k, ZincWriter::new(v.as_ref()))?;
                    if kv_pairs.peek().is_some() {
                        write!(f, " ")?;
                    };
                    Ok(())
                }
            }?;
        }
        write!(f, "}}")
    }
    pub fn to_trio<'b>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut dict_iter = self
            .inner
            .iter()
            .filter(|(_, v)| v.get_null().is_none())
            .peekable();
        while let Some((k, v)) = dict_iter.next() {
            let v_ref = v.as_ref();
            write!(f, "\"{}\":\"{}\"", k, JsonWriter::new(v_ref))?;
            if dict_iter.peek().is_some() {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HDict<'a, T> {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    fn _eq(&self, _other: &dyn HVal<'a, T>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h_number::HNumber;
    use std::rc::Rc;

    #[test]
    fn test_new_dict() {
        let dict: HDict<f64> = HDict::new();
        assert!(dict.inner.is_empty());
    }

    #[test]
    fn test_from_map() {
        let mut map: HashMap<String, HBox<f64>> = HashMap::new();
        let val: Rc<HNumber<f64>> = Rc::new(HNumber::new(42.0, None));
        map.insert("key1".to_string(), val);
        let dict = HDict::from_map(map.clone());
        assert_eq!(dict.inner.len(), 1);
        assert_eq!(
            dict.inner.get("key1").unwrap().get_number().unwrap().val(),
            42.0
        );
        assert_eq!(
            dict.inner.get("key1").unwrap().get_number().unwrap().unit(),
            &None
        );
    }

    #[test]
    fn test_haystack_type() {
        let dict: HDict<f64> = HDict::new();
        assert_eq!(dict.haystack_type(), HType::Dict);
    }

    #[test]
    fn test_eq() {
        let mut map1 = HashMap::new();
        let val1: Rc<dyn HVal<f64>> = Rc::new(HNumber::new(42.0, None));
        map1.insert("key1".to_string(), val1);
        let dict1 = HDict::from_map(map1);

        let mut map2 = HashMap::new();
        let val2: Rc<dyn HVal<f64>> = Rc::new(HNumber::new(42.0, None));
        map2.insert("key1".to_string(), val2);
        let dict2 = HDict::from_map(map2);

        assert!(!dict1._eq(&dict2));
    }
}
