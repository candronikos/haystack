use num::Float;
use std::collections::HashMap;
use crate::io::HBox;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

pub struct HDict<'a,T> {
    inner: HashMap<String, HBox<'a,T>>
}

pub type Dict<'a,T> = HDict<'a,T>;

const THIS_TYPE: HType = HType::Dict;

impl <'a,T: NumTrait + 'a>HDict<'a,T> {
    pub fn new() -> HDict<'a,T> {
        HDict { inner: HashMap::new() }
    }

    pub fn from_map(map: HashMap<String, HBox<'a,T>>) -> HDict<'a,T> {
        HDict { inner: map }
    }

    pub fn get(&self, key: &str) -> Option<&HBox<'a,T>> {
        self.inner.get(key)
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HDict<'a,T> {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{{")?;
        let inner = &self.inner;
        let mut kv_pairs = inner.into_iter()
            .filter(|(k,v)| v.get_null_val().is_none())
            .peekable();
        while let Some((k,v)) = kv_pairs.next() {
            match v.haystack_type() {
                HType::Remove => write!(buf,"-{}",k),
                HType::Marker => write!(buf,"{}",k),
                _ => {
                    write!(buf,"{}:",k)?;
                    v.to_zinc(buf)?;
                    if kv_pairs.peek().is_some() { write!(buf," ")?; };
                    Ok(())
                }
            }?;
        }
        write!(buf,"}}")
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
    }
    fn to_json(&self, _buf: &mut String) -> fmt::Result {
        unimplemented!()
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    fn _eq(&self, other: &dyn HVal<'a,T>) -> bool { false }
    set_get_method!(get_dict_val, HDict<'a,T>);
}

#[cfg(test)]
mod tests {
    use crate::{h_number::HNumber, HCast};
    use std::rc::Rc;
    use super::*;

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
        assert_eq!(dict.inner.get("key1").unwrap().get_number().unwrap().val(), 42.0);
        assert_eq!(dict.inner.get("key1").unwrap().get_number().unwrap().unit(), &None);
    }

    #[test]
    fn test_to_zinc() {
        let mut map = HashMap::new();
        let val1: Rc<dyn HVal<f64>> = Rc::new(HNumber::new(42.0, None));
        let val2: Rc<dyn HVal<f64>> = Rc::new(HNumber::new(3.14, None));
        map.insert("key1".to_string(), val1);
        map.insert("key2".to_string(), val2);
        let dict = HDict::<f64>::from_map(map);

        let mut buf = String::new();
        dict.to_zinc(&mut buf).unwrap();

        assert!(buf=="{key1:42 key2:3.14}" || buf=="{key2:3.14 key1:42}");
    }

    #[test]
    fn test_to_trio() {
        let mut map = HashMap::new();
        let val1: Rc<dyn HVal<f64>> = Rc::new(HNumber::new(42.0, None));
        map.insert("key1".to_string(), val1);
        let dict = HDict::from_map(map);

        let mut buf = String::new();
        dict.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "{key1:42}");
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_to_json() {
        let dict: HDict<f64> = HDict::new();
        let mut buf = String::new();
        dict.to_json(&mut buf).unwrap();
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