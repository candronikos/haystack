use std::str::FromStr;
use core::fmt::Display;
use num::Float;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self,Write};

use chrono::naive::NaiveDate;
use chrono::Datelike;

#[derive(Debug,PartialEq)]
pub struct HDate {
    inner: NaiveDate,
}

pub type Date = HDate;

const THIS_TYPE: HType = HType::Date;

impl HDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { inner: NaiveDate::from_ymd(year, month, day) }
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HDate {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{:0>4}-{:0>2}-{:0>2}",
            self.inner.year(),
            self.inner.month(),
            self.inner.day())
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"d:")?;
        let it: &dyn HVal<T> = self;
        it.to_zinc(buf)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_date_val,'a,T);
    set_get_method!(get_date_val, HDate);
}