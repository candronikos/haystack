use crate::{HVal,HType};
use std::fmt::{self,Write};

use url::Url;

pub struct HUri(Url);

pub type Uri = HUri;

const THIS_TYPE: HType = HType::Uri;

impl HVal for HUri {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        buf.push('`');
        buf.push_str(&self.0.to_string());
        buf.push('`');
        Ok(())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"u:{}",self.0)?;
        Ok(())
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}