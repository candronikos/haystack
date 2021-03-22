use crate::{HVal,HType};
use std::fmt::{self,Write};

use chrono::{NaiveDateTime as DT, NaiveDate};
use chrono::{Datelike,Timelike};

#[derive(Debug,PartialEq)]
pub struct HDateTime {
    inner: DT,
    tz: (chrono_tz::Tz, HOffset)
}

#[derive(Debug,PartialEq)]
pub enum HOffset {
    Fixed(chrono::FixedOffset),
    Variable(chrono::Duration),
    Utc
}

pub type DateTime = HDateTime;

const THIS_TYPE: HType = HType::DateTime;

impl HDateTime {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32, nano: u32, tz: (chrono_tz::Tz, HOffset)) -> Self {
        let inner = NaiveDate::from_ymd(year, month, day)
            .and_hms_nano(hour, min, sec, nano);

        Self { inner, tz }
    }
}

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