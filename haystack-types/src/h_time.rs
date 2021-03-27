use num::Float;
use crate::{HVal,HType};
use std::fmt::{self,Write,Display};
use std::str::FromStr;

use chrono::naive::NaiveTime;
use chrono::Timelike;

#[derive(Debug,PartialEq)]
pub struct HTime {
    inner: NaiveTime,
}

pub type Time = HTime;

const THIS_TYPE: HType = HType::Time;

impl HTime {
    pub fn new(hour: u32, minute: u32, second: u32, nano: u32) -> Self {
        Self { inner: NaiveTime::from_hms_nano(hour, minute, second, nano) }
    }
}

impl <'a,T:'a + Float + Display + FromStr>HVal<'a,T> for HTime {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{:0>2}:{:0>2}:{:0>2}.{}",self.inner.hour(),self.inner.minute(),
        self.inner.second(),self.inner.nanosecond())?;
        Ok(())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"h:")?;
        let it: &dyn HVal<T> = self;
        it.to_zinc(buf)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_time_val,'a,T);
    set_get_method!(get_time_val, HTime);
}