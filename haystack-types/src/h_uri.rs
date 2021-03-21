use crate::{HVal,HType};
use std::fmt::{self,Write};

use url::{Url,ParseError as UrlParseError};

#[derive(Debug,PartialEq)]
pub struct HUri(Url);

pub type Uri = HUri;

const THIS_TYPE: HType = HType::Uri;

impl HUri {
    pub fn new(input: &str) -> Result<HUri, UrlParseError> {
        let url = Url::parse(input)?;
        Ok(HUri(url))
    }
}

impl HVal for HUri {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        buf.push('`');
        buf.push_str(self.0.as_str());
        buf.push('`');
        Ok(())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"u:{}",self.0)?;
        Ok(())
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}