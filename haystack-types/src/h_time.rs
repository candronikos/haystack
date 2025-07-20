use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Write};

use chrono::Timelike;
use chrono::naive::NaiveTime;

#[derive(Clone, Debug, PartialEq)]
pub struct HTime {
    inner: NaiveTime,
}

pub type Time = HTime;

const THIS_TYPE: HType = HType::Time;

impl HTime {
    pub fn new(hour: u32, minute: u32, second: u32, nano: u32) -> Self {
        Self {
            inner: NaiveTime::from_hms_nano(hour, minute, second, nano),
        }
    }
    fn to_zinc(&self, buf: &mut String) -> fmt::Result {
        write!(
            buf,
            "{:0>2}:{:0>2}:{:0>2}.{}",
            self.inner.hour(),
            self.inner.minute(),
            self.inner.second(),
            self.inner.nanosecond()
        )?;
        Ok(())
    }
    fn to_trio(&self, buf: &mut String) -> fmt::Result {
        self.to_zinc(buf)
    }
    fn to_json(&self, buf: &mut String) -> fmt::Result {
        write!(buf, "h:")?;
        self.to_zinc(buf)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HTime {
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

    set_trait_eq_method!(get_time,'a,T);
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::naive::NaiveTime;

    #[test]
    fn test_new() {
        let time = HTime::new(12, 34, 56, 789_000_000);
        assert_eq!(
            time.inner,
            NaiveTime::from_hms_nano(12, 34, 56, 789_000_000)
        );
    }

    #[test]
    fn test_to_zinc() {
        let time = HTime::new(12, 34, 56, 789_000_000);
        let mut buf = String::new();
        time.to_zinc(&mut buf).unwrap();
        assert_eq!(buf, "12:34:56.789000000");
    }

    #[test]
    fn test_to_trio() {
        let time = HTime::new(12, 34, 56, 789_000_000);
        let mut buf = String::new();
        time.to_trio(&mut buf).unwrap();
        assert_eq!(buf, "12:34:56.789000000");
    }

    #[test]
    fn test_to_json() {
        let time = HTime::new(12, 34, 56, 789_000_000);
        let mut buf = String::new();
        time.to_json(&mut buf).unwrap();
        assert_eq!(buf, "h:12:34:56.789000000");
    }

    #[test]
    fn test_haystack_type() {
        let time = HTime::new(12, 34, 56, 789_000_000);
        let time_hval = HVal::<f64>::as_hval(&time);
        assert_eq!(time_hval.haystack_type(), HType::Time);
    }
}
