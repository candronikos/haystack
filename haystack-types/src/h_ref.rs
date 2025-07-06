use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Write};
use crate::common::{zinc_escape_str, escape_str_no_escape_unicode as escape_str};

#[derive(Clone,PartialEq)]
pub struct HRef {
    id: String,
    dis: Option<String>,
}

pub type Ref = HRef;

const THIS_TYPE: HType = HType::Ref;

impl HRef {
    pub fn new(id: String, dis: Option<String>) -> HRef {
        HRef { id, dis }
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HRef {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"@{}",self.id)?;
        match &self.dis {
            Some(dis) => {
                buf.push(' ');
                dis.chars().try_for_each(|c| { zinc_escape_str(c,buf) })?;
                Ok(())
            },
            None => Ok(()),
        }
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        HVal::<T>::to_zinc(self, buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"r:{}",self.id)?;
        match &self.dis {
            Some(dis) => {
                buf.push(' ');
                dis.chars().try_for_each(|c| { escape_str(c,buf) })?;
                Ok(())
            },
            None => Ok(()),
        }
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_ref_val,'a,T);
    set_get_method!(get_ref_val, HRef);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_href_new() {
        let href = HRef::new("id123".to_string(), Some("Building 1: \"Main\"".to_string()));
        assert_eq!(href.id, "id123");
        assert_eq!(href.dis, Some("Building 1: \"Main\"".to_string()));
        assert_ne!(href.dis, Some("Building 2: \"Main\"".to_string()));
    }

    #[test]
    fn test_href_to_zinc() {
        let href = HRef::new("id123".to_string(), Some("display".to_string()));
        let mut buf = String::new();
        let href_hval = HVal::<f64>::as_hval(&href);
        href_hval.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "@id123 display");
    }

    #[test]
    fn test_href_to_zinc_no_dis() {
        let href = HRef::new("id123".to_string(), None);
        let mut buf = String::new();
        let href_hval = HVal::<f64>::as_hval(&href);
        href_hval.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "@id123");
    }

    #[test]
    fn test_href_to_trio() {
        let href = HRef::new("id123".to_string(), Some("display".to_string()));
        let mut buf = String::new();
        let href_hval = HVal::<f64>::as_hval(&href);
        href_hval.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "@id123 display");
    }

    #[test]
    fn test_href_to_json() {
        let href = HRef::new("id123".to_string(), Some("display".to_string()));
        let mut buf = String::new();
        let href_hval = HVal::<f64>::as_hval(&href);
        href_hval.to_json(&mut buf).unwrap();
        assert_eq!(buf, "r:id123 display");
    }

    #[test]
    fn test_href_to_json_no_dis() {
        let href = HRef::new("id123".to_string(), None);
        let mut buf = String::new();
        let href_hval = HVal::<f64>::as_hval(&href);
        href_hval.to_json(&mut buf).unwrap();
        assert_eq!(buf, "r:id123");
    }

    #[test]
    fn test_haystack_type() {
        let href = HRef::new("id123".to_string(), None);
        let href_hval = HVal::<f64>::as_hval(&href);
        assert_eq!(href_hval.haystack_type(), HType::Ref);
    }
}
