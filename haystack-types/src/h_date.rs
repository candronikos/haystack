use crate::{HVal,HType};
use std::fmt::{self,Write};

use chrono::naive::NaiveDate;
use chrono::Datelike;

pub struct HDate {
    inner: NaiveDate,
}

pub type Date = HDate;

const THIS_TYPE: HType = HType::Date;

impl HVal for HDate {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{:0>4}-{:0>2}-{:0>2}",self.inner.year(),
        self.inner.month(),self.inner.day())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"d:")?;
        self.to_zinc(buf)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}