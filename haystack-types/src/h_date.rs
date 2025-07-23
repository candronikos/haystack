use crate::{HType, HVal, NumTrait};
use chrono::Datelike;
use chrono::naive::NaiveDate;
use std::fmt::{self, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct HDate {
    inner: NaiveDate,
}

pub type Date = HDate;

const THIS_TYPE: HType = HType::Date;

impl HDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self {
            inner: NaiveDate::from_ymd(year, month, day),
        }
    }
    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>4}-{:0>2}-{:0>2}",
            self.inner.year(),
            self.inner.month(),
            self.inner.day()
        )
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "d:")?;
        self.to_zinc(f)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HDate {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_date,'a,T);
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
    fn test_haystack_type() {
        let date = HDate::new(2023, 10, 5);
        let hval_type = HVal::<f64>::as_hval(&date);
        assert_eq!(hval_type.haystack_type(), HType::Date);
    }
}
