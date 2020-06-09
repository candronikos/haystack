use crate::{HVal,HType};
use std::fmt::{self,Write};

use chrono::{Utc,DateTime as DT};
use chrono::{Datelike,Timelike};

pub struct HDateTime {
    inner: DT<Utc>,
}

pub type DateTime = HDateTime;

const THIS_TYPE: HType = HType::DateTime;

impl HVal for HDateTime {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{:0>4}-{:0>2}-{:0>2}T",self.inner.year(),self.inner.month(),self.inner.day())?;
        write!(buf,"{:0>2}:{:0>2}:{:0>2}.{}",self.inner.hour(),self.inner.minute(),self.inner.second(),
        self.inner.nanosecond())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"t:")?;
        self.to_zinc(buf)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}