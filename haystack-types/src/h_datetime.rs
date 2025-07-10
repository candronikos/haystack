use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

use chrono::{Datelike, Timelike};
use chrono::{Duration, FixedOffset};
use chrono::{NaiveDate, NaiveDateTime as DT};

#[derive(Clone, Debug, PartialEq)]
pub struct HDateTime {
    inner: DT,
    // TODO: Implement timezones to work with `chrono_tz`
    // tz: (chrono_tz::Tz, HOffset)
    tz: (String, HOffset),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HOffset {
    Fixed(chrono::FixedOffset),
    Variable(chrono::Duration),
    Utc,
}

pub type DateTime = HDateTime;

const THIS_TYPE: HType = HType::DateTime;

impl HDateTime {
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
        tz: (String /* chrono_tz::Tz*/, HOffset),
    ) -> Self {
        let inner = NaiveDate::from_ymd(year, month, day).and_hms_nano(hour, min, sec, nano);

        Self { inner, tz }
    }
    pub fn val(&self) -> DT {
        self.inner
    }
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(
            buf,
            "{:0>4}-{:0>2}-{:0>2}T",
            self.inner.year(),
            self.inner.month(),
            self.inner.day()
        )?;
        write!(
            buf,
            "{:0>2}:{:0>2}:{:0>2}.{}",
            self.inner.hour(),
            self.inner.minute(),
            self.inner.second(),
            self.inner.nanosecond()
        )
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "t:")?;
        self.to_zinc(buf)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HDateTime {
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_trio(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        self.to_json(buf)
    }
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_datetime_val,'a,T);
    set_get_method!(get_datetime_val, HDateTime);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tz = ("UTC".to_string(), HOffset::Utc);
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz.clone());
        assert_eq!(datetime.inner.year(), 2023);
        assert_eq!(datetime.inner.month(), 10);
        assert_eq!(datetime.inner.day(), 5);
        assert_eq!(datetime.inner.hour(), 14);
        assert_eq!(datetime.inner.minute(), 30);
        assert_eq!(datetime.inner.second(), 45);
        assert_eq!(datetime.inner.nanosecond(), 123456789);
        assert_eq!(datetime.tz, tz);
    }

    #[test]
    fn test_to_zinc() {
        let tz = ("UTC".to_string(), HOffset::Utc);
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz);
        let mut buf = String::new();
        datetime.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "2023-10-05T14:30:45.123456789");
    }

    #[test]
    fn test_to_trio() {
        let tz = ("UTC".to_string(), HOffset::Utc);
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz);
        let mut buf = String::new();
        datetime.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "2023-10-05T14:30:45.123456789");
    }

    #[test]
    fn test_to_json() {
        let tz = ("UTC".to_string(), HOffset::Utc);
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz);
        let mut buf = String::new();
        datetime.to_json(&mut buf).unwrap();
        assert_eq!(buf, "t:2023-10-05T14:30:45.123456789");
    }

    #[test]
    fn test_with_fixed_offset() {
        let tz = (
            "FixedOffset".to_string(),
            HOffset::Fixed(FixedOffset::east(3600)),
        );
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 0, tz.clone());
        assert_eq!(datetime.tz, tz);
    }

    #[test]
    fn test_with_variable_offset() {
        let tz = (
            "VariableOffset".to_string(),
            HOffset::Variable(Duration::hours(2)),
        );
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 0, tz.clone());
        assert_eq!(datetime.tz, tz);
    }

    #[test]
    fn test_haystack_type() {
        let tz = ("UTC".to_string(), HOffset::Utc);
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz);
        let hval_type = HVal::<f64>::as_hval(&datetime);
        assert_eq!(hval_type.haystack_type(), HType::DateTime);
    }
}
