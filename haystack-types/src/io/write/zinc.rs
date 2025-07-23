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

pub struct ZincWriter<'a, T>
where
    T: ?Sized + 'a,
{
    value: &'a T,
}

impl<'a, T: ?Sized> ZincWriter<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }
}

impl<'a, T: ?Sized> Display for ZincWriter<'a, T>
where
    T: ZincWritable + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.to_zinc(f)
    }
}

pub trait ZincWritable {
    fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_zinc_writable {
    ($h_type:ty) => {
        impl ZincWritable for $h_type {
            fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$h_type>::to_zinc(self, f)
            }
        }
    };
    ($h_type:ty, $num_trait:ident) => {
        impl<'a, T: $num_trait + 'a> ZincWritable for $h_type {
            fn to_zinc(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$h_type>::to_zinc(self, f)
            }
        }
    };
}

impl_zinc_writable!(HNull);
impl_zinc_writable!(HMarker);
impl_zinc_writable!(HRemove);
impl_zinc_writable!(HNA);
impl_zinc_writable!(HBool);
impl_zinc_writable!(HStr);
impl_zinc_writable!(HXStr);
impl_zinc_writable!(HUri);
impl_zinc_writable!(HDate);
impl_zinc_writable!(HDateTime);
impl_zinc_writable!(HTime);
impl_zinc_writable!(HRef);
impl_zinc_writable!(HSymbol);
impl_zinc_writable!(HCoord<T>, NumTrait);
impl_zinc_writable!(HNumber<T>, NumTrait);
impl_zinc_writable!(HDict<'a, T>, NumTrait);
impl_zinc_writable!(HList<'a, T>, NumTrait);
impl_zinc_writable!(HGrid<'a, T>, NumTrait);

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
        write!(buf, "{}", ZincWriter::new(&null)).unwrap();
        assert_eq!(buf, "N");
    }

    #[test]
    fn test_marker() {
        let marker = HMarker;
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&marker)).unwrap();
        assert_eq!(buf, "M");
    }

    #[test]
    fn test_remove() {
        let remove = HRemove;
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&remove)).unwrap();
        assert_eq!(buf, "R");
    }

    #[test]
    fn test_bool_true() {
        let hbool = HBool(true);
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&hbool)).unwrap();
        assert_eq!(buf, "T");
    }

    #[test]
    fn test_bool_false() {
        let hbool = HBool(false);
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&hbool)).unwrap();
        assert_eq!(buf, "F");
    }

    #[test]
    fn test_coord() {
        let coord = HCoord::new(10.5, 20.5);
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&coord)).unwrap();
        assert_eq!(buf, "C(10.5,20.5)");
    }

    #[test]
    fn test_date() {
        let date = HDate::new(2023, 10, 5).unwrap();
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&date)).unwrap();
        assert_eq!(buf, "2023-10-05");
    }

    #[test]
    fn test_number_with_unit() {
        let unit = Some("m".to_owned().into());
        let number = HNumber::new(42.0, unit);
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&number)).unwrap();
        assert_eq!(buf, "42m");
    }

    #[test]
    fn test_symbol() {
        let symbol = HSymbol::new("example".to_string());
        let mut buf = String::new();
        let symbol_hval = HVal::<f64>::as_hval(&symbol);
        write!(buf, "{}", ZincWriter::new(symbol_hval)).unwrap();
        assert_eq!(buf, "^example");
    }

    #[test]
    fn test_number_without_unit() {
        let number = HNumber::new(3.14, None);
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&number)).unwrap();
        assert_eq!(buf, "3.14");
    }

    #[test]
    fn test_time() {
        let time = HTime::new(12, 34, 56, 789_000_000).unwrap();
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&time)).unwrap();
        assert_eq!(buf, "12:34:56.789000000");
    }

    #[test]
    fn test_datetime() {
        let tz = HTimezone::default();
        let datetime = HDateTime::new(2023, 10, 5, 14, 30, 45, 123456789, tz.clone()).unwrap();
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&datetime)).unwrap();
        assert_eq!(buf, "2023-10-05T14:30:45.123456789");
    }

    #[test]
    fn test_dict() {
        let mut map = HashMap::new();
        let val1 = HNumber::new(42.0, None);
        let val2 = HNumber::new(3.14, None);
        map.insert("key1".to_string(), val1.to_hbox());
        map.insert("key2".to_string(), val2.to_hbox());
        let dict = HDict::<f64>::from_map(map);

        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&dict)).unwrap();

        assert!(buf == "{key1:42 key2:3.14}" || buf == "{key2:3.14 key1:42}");
    }

    #[test]
    fn test_na() {
        let na = HNA;
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&na)).unwrap();
        assert_eq!(buf, "NA");
    }

    #[test]
    fn test_str() {
        let hstr = HStr::new("hello".into());
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&hstr)).unwrap();
        assert_eq!(buf, "\"hello\"");
    }

    #[test]
    fn test_str_escaped_chars() {
        //let hstr = HStr::new("\b \f \n \r \t \" \\ $ \u{263A}".into());
        let hstr = HStr::new("\x08 \x0C \n \r \t \" \\ $ \u{263A} ☺".into());
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&hstr)).unwrap();
        assert_eq!(buf, "\"\\b \\f \\n \\r \\t \\\" \\\\ \\$ \u{263A} ☺\"");
    }

    #[test]
    fn test_uri() {
        let input = "https://example.com";
        let huri = HUri::new(input).unwrap();
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&huri)).unwrap();
        assert_eq!(buf, "`https://example.com/`");
    }

    #[test]
    fn test_href() {
        let href = HRef::new("id123".to_string(), Some("display".to_string()));
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&href)).unwrap();
        assert_eq!(buf, "@id123 display");
    }

    #[test]
    fn test_href_no_dis() {
        let href = HRef::new("id123".to_string(), None);
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&href)).unwrap();
        assert_eq!(buf, "@id123");
    }

    #[test]
    fn test_xstr() {
        let xhstr = HXStr::new("custom".to_string(), "hello".into());
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&xhstr)).unwrap();
        assert_eq!(buf, "custom(\"hello\")");
    }

    #[test]
    fn test_list() {
        let mut dict_element = HDict::new();
        dict_element.set("key".into(), HNumber::new(1f64, None).to_hbox());

        let vec = vec![
            HNumber::new(1f64, None).to_hbox(),
            HNumber::new(2f64, Some("m".to_owned().into())).to_hbox(),
            HStr::new("three$".into()).to_hbox(),
            HUri::new("http://example.com").unwrap().to_hbox(),
            dict_element.to_hbox(),
        ];
        let hlist = HList::from_vec(vec);
        let mut buf = String::new();
        write!(buf, "{}", ZincWriter::new(&hlist)).unwrap();
        assert_eq!(buf, "[1, 2m, \"three\\$\", `http://example.com/`, {key:1}]");
    }
}
