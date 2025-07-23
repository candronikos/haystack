use crate::{HType, HVal, NumTrait};
use std::fmt::{self, Display};

use chrono::Timelike;
use chrono::naive::NaiveTime;

#[derive(Clone, Debug, PartialEq)]
pub struct HTime {
    inner: NaiveTime,
}

pub type Time = HTime;

const THIS_TYPE: HType = HType::Time;

#[derive(Debug)]
pub enum HTimeErr {
    InvalidDate,
    ShouldNeverHappen,
}

impl Display for HTimeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HTimeErr::InvalidDate => write!(f, "Invalid time"),
            HTimeErr::ShouldNeverHappen => write!(f, "This should never happen"),
        }
    }
}

impl HTime {
    pub fn new(hour: u32, minute: u32, second: u32, nano: u32) -> Result<Self, HTimeErr> {
        Ok(Self {
            inner: NaiveTime::from_hms_nano_opt(hour, minute, second, nano)
                .ok_or(HTimeErr::InvalidDate)?,
        })
    }
    pub fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}",
            self.inner.hour(),
            self.inner.minute(),
            self.inner.second()
        )?;
        if self.inner.nanosecond() != 0 {
            write!(f, ".{:0>9}", self.inner.nanosecond())?;
        }
        Ok(())
    }
    pub fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_zinc(f)
    }
    pub fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "h:")?;
        self.to_zinc(f)
    }
}

impl<'a, T: NumTrait + 'a> HVal<'a, T> for HTime {
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
        let time = HTime::new(12, 34, 56, 789_000_000).unwrap();
        assert_eq!(
            time.inner,
            NaiveTime::from_hms_nano_opt(12, 34, 56, 789_000_000).unwrap()
        );
    }

    #[test]
    fn test_haystack_type() {
        let time = HTime::new(12, 34, 56, 789_000_000).unwrap();
        let time_hval = HVal::<f64>::as_hval(&time);
        assert_eq!(time_hval.haystack_type(), HType::Time);
    }
}
