use crate::h_time::HTime;
use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Display};

use crate::h_date::HDate;
use chrono::offset::LocalResult;
use chrono::{Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Timelike};
use chrono_tz::{OffsetComponents, Tz};

#[derive(Clone, Debug, PartialEq)]
pub struct HDateTime {
    inner: NaiveDateTime,
    tz: HTimezone,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HTimezone {
    offset: chrono::FixedOffset,
    id: Tz,
}

impl Default for HTimezone {
    fn default() -> Self {
        Self {
            offset: FixedOffset::east_opt(0).unwrap(),
            id: Tz::UTC,
        }
    }
}

pub trait IntoTimezone {
    fn into_timezone(self) -> HTimezone;
}

impl IntoTimezone for (FixedOffset, Tz) {
    fn into_timezone(self) -> HTimezone {
        HTimezone {
            offset: self.0,
            id: self.1,
        }
    }
}

impl Display for HTimezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.name())
    }
}

pub type DateTime = HDateTime;

const THIS_TYPE: HType = HType::DateTime;

#[derive(Debug)]
pub enum TZError {
    InvalidFormat,
    UnknownTimezone,
    ParseError(chrono_tz::ParseError),
    Nonexistent,
    DateConstruction,
    TimeConstruction,
}

impl HDateTime {
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
        tz: HTimezone,
    ) -> Result<Self, TZError> {
        let inner = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or(TZError::DateConstruction)?
            .and_hms_nano_opt(hour, min, sec, nano)
            .ok_or(TZError::TimeConstruction)?;
        Ok(Self { inner, tz })
    }
    pub fn val(&self) -> NaiveDateTime {
        self.inner
    }
    pub fn date(&self) -> HDate {
        HDate::new(self.inner.year(), self.inner.month(), self.inner.day())
    }

    pub fn time(&self) -> HTime {
        HTime::new(
            self.inner.hour(),
            self.inner.minute(),
            self.inner.second(),
            self.inner.nanosecond(),
        )
    }

    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    pub fn month(&self) -> u32 {
        self.inner.month()
    }

    pub fn day(&self) -> u32 {
        self.inner.day()
    }

    pub fn hour(&self) -> u32 {
        self.inner.hour()
    }

    pub fn minute(&self) -> u32 {
        self.inner.minute()
    }

    pub fn second(&self) -> u32 {
        self.inner.second()
    }

    pub fn nanosecond(&self) -> u32 {
        self.inner.nanosecond()
    }

    pub fn is_dst(&self) -> bool {
        let local_result = self.tz.id.from_local_datetime(&self.inner);
        match local_result {
            LocalResult::Single(one) => one.offset().dst_offset() != Duration::zero(),
            _ => true,
        }
    }

    pub fn tz(&self) -> &HTimezone {
        &self.tz
    }

    pub fn tz_id(&self) -> chrono_tz::Tz {
        self.tz.id
    }

    pub fn offset(&self) -> FixedOffset {
        self.tz.offset
    }

    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>4}-{:0>2}-{:0>2}T",
            self.inner.year(),
            self.inner.month(),
            self.inner.day()
        )?;
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}.{}",
            self.inner.hour(),
            self.inner.minute(),
            self.inner.second(),
            self.inner.nanosecond()
        )
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t:")?;
        self.to_zinc(f)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HDateTime {
    fn haystack_type(&self) -> HType {
        THIS_TYPE
    }

    set_trait_eq_method!(get_datetime,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tz = HTimezone::default();
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz.clone()).unwrap();
        assert_eq!(datetime.inner.year(), 2023);
        assert_eq!(datetime.inner.month(), 10);
        assert_eq!(datetime.inner.day(), 5);
        assert_eq!(datetime.inner.hour(), 14);
        assert_eq!(datetime.inner.minute(), 30);
        assert_eq!(datetime.inner.second(), 45);
        assert_eq!(datetime.inner.nanosecond(), 123456789);
        assert_eq!(datetime.tz(), &tz);
    }

    #[test]
    fn test_with_fixed_offset() {
        let tz = HTimezone::default();
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 0, tz.clone()).unwrap();
        assert_eq!(datetime.tz(), &tz);
    }

    #[test]
    fn test_with_variable_offset() {
        let tz = HTimezone::default();
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 0, tz.clone()).unwrap();
        assert_eq!(datetime.tz(), &tz);
    }

    #[test]
    fn test_haystack_type() {
        let tz = HTimezone::default();
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz).unwrap();
        let hval_type = HVal::<f64>::as_hval(&datetime);
        assert_eq!(hval_type.haystack_type(), HType::DateTime);
    }
}
