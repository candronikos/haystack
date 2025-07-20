use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

use url::{ParseError as UrlParseError, Url};

#[derive(Clone, Debug, PartialEq)]
pub struct HUri(Url);

pub type Uri = HUri;

const THIS_TYPE: HType = HType::Uri;

impl HUri {
    pub fn new(input: &str) -> Result<HUri, UrlParseError> {
        let url = Url::parse(input)?;
        Ok(HUri(url))
    }
    pub fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        buf.push('`');
        buf.push_str(self.0.as_str());
        buf.push('`');
        Ok(())
    }
    pub fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    pub fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "u:{}", self.0)?;
        Ok(())
    }
    pub fn to_owned_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HUri {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_uri,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let input = "https://example.com";
        let huri = HUri::new(input);
        assert!(huri.is_ok());
        assert_eq!(huri.unwrap().0.as_str(), input);
    }

    #[test]
    fn test_new_invalid() {
        let input = "not-a-valid-url";
        let huri = HUri::new(input);
        assert!(huri.is_err());
    }

    #[test]
    fn test_to_zinc() {
        let input = "https://example.com";
        let huri = HUri::new(input).unwrap();
        let mut buf = String::new();
        huri.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "`https://example.com`");
    }

    #[test]
    fn test_to_trio() {
        let input = "https://example.com";
        let huri = HUri::new(input).unwrap();
        let mut buf = String::new();
        huri.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "`https://example.com`");
    }

    #[test]
    fn test_to_json() {
        let input = "https://example.com";
        let huri = HUri::new(input).unwrap();
        let mut buf = String::new();
        huri.to_json(&mut buf).unwrap();
        assert_eq!(buf, "u:https://example.com");
    }

    #[test]
    fn test_haystack_type() {
        let input = "https://example.com";
        let huri = HUri::new(input).unwrap();
        assert_eq!(HVal::<f64>::as_hval(&huri).haystack_type(), HType::Uri);
    }
}
