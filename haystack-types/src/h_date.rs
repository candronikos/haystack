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
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"{:0>4}-{:0>2}-{:0>2}",
            self.inner.year(),
            self.inner.month(),
            self.inner.day())
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf,"d:")?;
        self.to_zinc(buf)
    }
}

impl <'a,T: NumTrait + 'a>HVal<'a,T> for HDate {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_trio(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        self.to_json(buf)
    }
    fn haystack_type(&self) -> HType { THIS_TYPE }

    set_trait_eq_method!(get_date_val,'a,T);
    set_get_method!(get_date_val, HDate);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let date = HDate::new(2023, 10, 5);
        assert_eq!(date.inner, NaiveDate::from_ymd(2023, 10, 5));
    }

    #[test]
    fn test_to_zinc() {
        let date = HDate::new(2023, 10, 5);
        let mut buf = String::new();
        date.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "2023-10-05");
    }

    #[test]
    fn test_to_trio() {
        let date = HDate::new(2023, 10, 5);
        let mut buf = String::new();
        date.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "2023-10-05");
    }

    #[test]
    fn test_to_json() {
        let date = HDate::new(2023, 10, 5);
        let mut buf = String::new();
        date.to_json(&mut buf).unwrap();
        assert_eq!(buf, "d:2023-10-05");
    }

    #[test]
    fn test_haystack_type() {
        let date = HDate::new(2023, 10, 5);
        let hval_type = HVal::<f64>::as_hval(&date);
        assert_eq!(hval_type.haystack_type(), HType::Date);
    }
}