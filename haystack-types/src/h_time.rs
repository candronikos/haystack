use crate::{HVal,HType};
use std::fmt::{self,Write};

use chrono::naive::NaiveTime;
use chrono::Timelike;

pub struct HTime {
    inner: NaiveTime,
}

pub type Time = HTime;

const THIS_TYPE: HType = HType::Time;

impl HVal for HTime {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{:0>2}:{:0>2}:{:0>2}.{}",self.inner.hour(),self.inner.minute(),
        self.inner.second(),self.inner.nanosecond())?;
        Ok(())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"h:")?;
        self.to_zinc(buf)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }
}