use crate::common::{escape_str_no_escape_unicode as escape_str, zinc_escape_str};
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct HRef {
    pub id: String,
    pub dis: Option<String>,
}

pub type Ref = HRef;

const THIS_TYPE: HType = HType::Ref;

impl HRef {
    pub fn new(id: String, dis: Option<String>) -> HRef {
        HRef { id, dis }
    }
    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.id)?;
        match &self.dis {
            Some(dis) => {
                f.write_char(' ');
                dis.chars().try_for_each(|c| zinc_escape_str(c, f))?;
                Ok(())
            }
            None => Ok(()),
        }
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r:{}", self.id)?;
        match &self.dis {
            Some(dis) => {
                f.write_char(' ');
                dis.chars().try_for_each(|c| escape_str(c, f))?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HRef {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_ref,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_href_new() {
        let href = HRef::new(
            "id123".to_string(),
            Some("Building 1: \"Main\"".to_string()),
        );
        assert_eq!(href.id, "id123");
        assert_eq!(href.dis, Some("Building 1: \"Main\"".to_string()));
        assert_ne!(href.dis, Some("Building 2: \"Main\"".to_string()));
    }

    #[test]
    fn test_haystack_type() {
        let href = HRef::new("id123".to_string(), None);
        let href_hval = HVal::<f64>::as_hval(&href);
        assert_eq!(href_hval.haystack_type(), HType::Ref);
    }
}
