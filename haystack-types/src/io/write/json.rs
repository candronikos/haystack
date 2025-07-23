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

pub struct JsonWriter<'a, T>
where
    T: ?Sized + 'a,
{
    value: &'a T,
}

impl<'a, T: ?Sized> JsonWriter<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }
}

impl<'a, T: ?Sized> Display for JsonWriter<'a, T>
where
    T: JsonWritable + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.to_json(f)
    }
}

pub trait JsonWritable {
    fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_json_writable {
    ($h_type:ty) => {
        impl JsonWritable for $h_type {
            fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$h_type>::to_json(self, f)
            }
        }
    };
    ($h_type:ty, $num_trait:ident) => {
        impl<'a, T: $num_trait + 'a> JsonWritable for $h_type {
            fn to_json(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$h_type>::to_json(self, f)
            }
        }
    };
}

impl_json_writable!(HNull);
impl_json_writable!(HMarker);
impl_json_writable!(HRemove);
impl_json_writable!(HNA);
impl_json_writable!(HBool);
impl_json_writable!(HStr);
impl_json_writable!(HXStr);
impl_json_writable!(HUri);
impl_json_writable!(HDate);
impl_json_writable!(HDateTime);
impl_json_writable!(HTime);
impl_json_writable!(HRef);
impl_json_writable!(HSymbol);
impl_json_writable!(HCoord<T>, NumTrait);
impl_json_writable!(HNumber<T>, NumTrait);
impl_json_writable!(HDict<'a, T>, NumTrait);
impl_json_writable!(HList<'a, T>, NumTrait);
// TODO: Implement tests for grid->toJSON
impl_json_writable!(HGrid<'a, T>, NumTrait);

#[cfg(test)]
mod tests {
    use crate::{HVal, h_datetime::HTimezone};
    use std::fmt::Write;

    use super::*;

    #[test]
    fn test_date() {
        let date = HDate::new(2023, 10, 5);
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&date)).unwrap();
        assert_eq!(buf, "d:2023-10-05");
        buf.clear();

        let mut buf = String::new();
        let date = HDate::new(2023, 12, 25);
        write!(buf, "{}", JsonWriter::new(&date)).unwrap();
        assert_eq!(buf, "d:2023-12-25");
    }

    #[test]
    fn test_datetime() {
        let tz = HTimezone::default();
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz.clone()).unwrap();
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&datetime)).unwrap();
        assert_eq!(buf, "t:2023-10-05T14:30:45.123456789");
    }

    #[test]
    fn test_null() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HNull)).unwrap();
        assert_eq!(buf, "null");
    }

    #[test]
    fn test_marker() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HMarker)).unwrap();
        assert_eq!(buf, "m:");
    }

    #[test]
    fn test_remove() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HRemove)).unwrap();
        assert_eq!(buf, "-:");
    }

    #[test]
    fn test_na() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HNA)).unwrap();
        assert_eq!(buf, "z:");
    }

    #[test]
    fn test_bool() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HBool(true))).unwrap();
        assert_eq!(buf, "true");

        buf.clear();
        write!(buf, "{}", JsonWriter::new(&HBool(false))).unwrap();
        assert_eq!(buf, "false");
    }

    #[test]
    fn test_coord() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&coord)).unwrap();
        assert_eq!(buf, "c:10.5,20.5");
    }

    #[test]
    fn test_str() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HStr::new("hello world".into()))).unwrap();
        assert_eq!(buf, "hello world");

        let hstr = HStr::new("hello".into());
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&hstr)).unwrap();
        assert_eq!(buf, "hello");

        let hstr_with_colon = HStr::new("key:value".into());
        let mut buf_with_colon = String::new();
        write!(buf_with_colon, "{}", JsonWriter::new(&hstr_with_colon)).unwrap();
        assert_eq!(buf_with_colon, "s:key:value");
    }

    #[test]
    fn test_number() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HNumber::new(42.0, None))).unwrap();
        assert_eq!(buf, "n:42");

        buf.clear();
        write!(
            buf,
            "{}",
            JsonWriter::new(&HNumber::new(42.2, Some("°F".to_owned().into())))
        )
        .unwrap();
        assert_eq!(buf, "n:42.2 °F");
    }

    #[test]
    fn test_ref() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HRef::new("site1".into(), None))).unwrap();
        assert_eq!(buf, "r:site1");

        buf.clear();
        write!(
            buf,
            "{}",
            JsonWriter::new(&HRef::new("site1".into(), Some("Site 1".into())))
        )
        .unwrap();
        assert_eq!(buf, "r:site1 Site 1");
    }

    #[test]
    fn test_symbol() {
        let symbol = HSymbol::new("example".to_string());
        let mut buf = String::new();
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        write!(buf, "{}", JsonWriter::new(symbol_hval)).unwrap();
        assert_eq!(buf, "y:example");
    }

    #[test]
    fn test_uri() {
        let mut buf = String::new();
        write!(
            buf,
            "{}",
            JsonWriter::new(&HUri::new("http://example.com").unwrap())
        )
        .unwrap();
        assert_eq!(buf, "u:http://example.com/");
    }

    #[test]
    fn test_time() {
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&HTime::new(14, 30, 0, 0))).unwrap();
        assert_eq!(buf, "h:14:30:00");

        let time = HTime::new(12, 34, 56, 789_000_000);
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&time)).unwrap();
        assert_eq!(buf, "h:12:34:56.789000000");
    }

    #[test]
    fn test_xstr() {
        let xhstr = HXStr::new("custom".to_string(), "hello".into());
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&xhstr)).unwrap();
        assert_eq!(buf, "x:custom:hello");
    }

    #[test]
    fn test_dict() {
        let mut buf = String::new();
        let mut dict = HDict::<f64>::new();
        dict.set("id".into(), HRef::new("site1".into(), None).to_hbox());
        dict.set("name".into(), HStr::new("Site 1".into()).to_hbox());
        write!(buf, "{}", JsonWriter::new(&dict)).unwrap();
        assert!(buf.starts_with("{"));
        assert!(buf.ends_with("}"));
        assert!(buf.contains("\"id\""));
        assert!(buf.contains("\"name\""));

        buf.clear();
        let dict: HDict<f64> = HDict::new();
        let mut buf = String::new();
        write!(buf, "{}", JsonWriter::new(&dict)).unwrap();
        assert_eq!(buf, "{}");
    }

    #[test]
    fn test_list() {
        let mut buf = String::new();
        let mut list = HList::new();
        list.push(HStr::new("item1".into()).to_hbox());
        list.push(HNumber::new(42.0, None).to_hbox());
        write!(buf, "{}", JsonWriter::new(&list)).unwrap();

        assert_eq!(buf, "[\"item1\",\"n:42\"]");
        assert!(buf.starts_with("["));
        assert!(buf.ends_with("]"));
    }
}
