use std::fmt::{self, Display};

use crate::{
    h_bool::HBool,
    h_coord::HCoord,
    h_date::HDate,
    h_datetime::HDateTime,
    h_dict::HDict,
    h_grid::HGrid,
    h_list::HList,
    h_marker::HMarker,
    h_na::HNA,
    h_null::HNull,
    h_number::{HNumber, NumTrait},
    h_ref::HRef,
    h_remove::HRemove,
    h_str::HStr,
    h_symbol::HSymbol,
    h_time::HTime,
    h_uri::HUri,
    h_xstr::HXStr,
};

pub struct TrioWriter<'a, T>
where
    T: ?Sized + 'a,
{
    value: &'a T,
}

impl<'a, T: ?Sized> TrioWriter<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }
}

impl<'a, T: ?Sized> Display for TrioWriter<'a, T>
where
    T: TrioWritable + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.to_trio(f)
    }
}

pub trait TrioWritable {
    fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_trio_writable {
    ($h_type:ty) => {
        impl TrioWritable for $h_type {
            fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$h_type>::to_trio(self, f)
            }
        }
    };
    ($h_type:ty, $num_trait:ident) => {
        impl<'a, T: $num_trait + 'a> TrioWritable for $h_type {
            fn to_trio(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$h_type>::to_trio(self, f)
            }
        }
    };
}

impl_trio_writable!(HNull);
impl_trio_writable!(HMarker);
impl_trio_writable!(HRemove);
impl_trio_writable!(HNA);
impl_trio_writable!(HBool);
impl_trio_writable!(HStr);
impl_trio_writable!(HXStr);
impl_trio_writable!(HUri);
impl_trio_writable!(HDate);
impl_trio_writable!(HDateTime);
impl_trio_writable!(HTime);
impl_trio_writable!(HRef);
impl_trio_writable!(HSymbol);
impl_trio_writable!(HCoord<T>, NumTrait);
impl_trio_writable!(HNumber<T>, NumTrait);
impl_trio_writable!(HDict<'a, T>, NumTrait);
impl_trio_writable!(HList<'a, T>, NumTrait);
impl_trio_writable!(HGrid<'a, T>, NumTrait);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt::Write;

    use crate::{HVal, h_datetime::HTimezone};

    use super::*;

    #[test]
    fn test_null() {
        let null = HNull;
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&null)).unwrap();
        assert_eq!(buf, "N");
    }

    #[test]
    fn test_remove() {
        let remove = HRemove;
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&remove)).unwrap();
        assert_eq!(buf, "R");
    }

    #[test]
    fn test_coord() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&coord)).unwrap();
        assert_eq!(buf, "C(10.5,20.5)");
    }

    #[test]
    fn test_date() {
        let date = HDate::new(2023, 10, 5);
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&date)).unwrap();
        assert_eq!(buf, "2023-10-05");
    }

    #[test]
    fn test_sumbol() {
        let symbol = HSymbol::new("example".to_string());
        let mut buf = String::new();
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        write!(buf, "{}", TrioWriter::new(symbol_hval)).unwrap();
        assert_eq!(buf, "^example");
    }

    #[test]
    fn test_number() {
        let unit = Some("kg".to_owned().into());
        let number = HNumber::new(100.0, unit);
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&number)).unwrap();
        assert_eq!(buf, "100kg");
    }

    #[test]
    fn test_datetime() {
        let tz = HTimezone::default();
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz.clone()).unwrap();
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&datetime)).unwrap();
        assert_eq!(buf, "2023-10-05T14:30:45.123456789");
    }

    #[test]
    fn test_time() {
        let time = HTime::new(12, 34, 56, 789_000_000);
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&time)).unwrap();
        assert_eq!(buf, "12:34:56.789000000");
    }

    #[test]
    fn test_dict() {
        let mut map = HashMap::new();
        let val1 = HNumber::new(42.0, None);
        map.insert("key1".to_string(), val1.to_hbox());
        let dict = HDict::from_map(map);

        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&dict)).unwrap();
    }

    #[test]
    fn test_marker() {
        let marker = HMarker;
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&marker)).unwrap();
        assert_eq!(buf, "M");
    }

    #[test]
    fn test_na() {
        let na = HNA;
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&na)).unwrap();
        assert_eq!(buf, "NA");
    }

    #[test]
    fn test_str() {
        let hstr = HStr::new("hello".into());
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&hstr)).unwrap();
        assert_eq!(buf, "\"hello\"");
    }

    #[test]
    fn test_uri() {
        let input = "https://example.com";
        let huri = HUri::new(input).unwrap();
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&huri)).unwrap();
        assert_eq!(buf, "`https://example.com/`");
    }

    #[test]
    fn test_href() {
        let href = HRef::new("id123".to_string(), Some("display".to_string()));
        let mut buf = String::new();
        let href_hval = HVal::<f64>::as_hval(&href);
        write!(buf, "{}", TrioWriter::new(href_hval)).unwrap();
        assert_eq!(buf, "@id123 display");
    }

    #[test]
    fn test_xstr() {
        let xhstr = HXStr::new("custom".to_string(), "hello".into());
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&xhstr)).unwrap();
        assert_eq!(buf, "custom(\"hello\")");
    }

    #[test]
    fn test_list() {
        let vec = vec![
            HNumber::new(1f64, None).to_hbox(),
            HNumber::new(2f64, None).to_hbox(),
        ];
        let hlist = HList::from_vec(vec);
        let mut buf = String::new();
        write!(buf, "{}", TrioWriter::new(&hlist)).unwrap();
        assert_eq!(buf, "[1, 2]");
    }
}
