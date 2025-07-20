use std::fmt;

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

pub trait JsonWriter<'a, T: NumTrait + 'a> {
    fn to_json(&self, buf: &mut String) -> fmt::Result;
}

macro_rules! impl_json_writer {
    ($h_type:ty) => {
        impl<'a, T: NumTrait + 'a> JsonWriter<'a, T> for $h_type {
            fn to_json(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_json(self, buf)
            }
        }
    };
    ($h_type:ty, $num_trait:ident) => {
        impl<'a, T: $num_trait + 'a> JsonWriter<'a, T> for $h_type {
            fn to_json(&self, buf: &mut String) -> fmt::Result {
                <$h_type>::to_json(self, buf)
            }
        }
    };
}

impl_json_writer!(HNull);
impl_json_writer!(HMarker);
impl_json_writer!(HRemove);
impl_json_writer!(HNA);
impl_json_writer!(HBool);
impl_json_writer!(HStr);
impl_json_writer!(HXStr);
impl_json_writer!(HUri);
impl_json_writer!(HDate);
impl_json_writer!(HDateTime);
impl_json_writer!(HTime);
impl_json_writer!(HRef);
impl_json_writer!(HSymbol);
impl_json_writer!(HCoord<T>, NumTrait);
impl_json_writer!(HNumber<T>, NumTrait);
impl_json_writer!(HDict<'a, T>, NumTrait);
impl_json_writer!(HList<'a, T>, NumTrait);
impl_json_writer!(HGrid<'a, T>, NumTrait);
#[cfg(test)]
mod tests {
    use crate::HVal;

    use super::*;

    #[test]
    fn test_null() {
        let mut buf = String::new();
        HNull.to_json(&mut buf).unwrap();
        assert_eq!(buf, "null");
    }

    #[test]
    fn test_marker() {
        let mut buf = String::new();
        HMarker.to_json(&mut buf).unwrap();
        assert_eq!(buf, "m:");
    }

    #[test]
    fn test_remove() {
        let mut buf = String::new();
        HRemove.to_json(&mut buf).unwrap();
        assert_eq!(buf, "-:");
    }

    #[test]
    fn test_na() {
        let mut buf = String::new();
        HNA.to_json(&mut buf).unwrap();
        assert_eq!(buf, "z:");
    }

    #[test]
    fn test_bool() {
        let mut buf = String::new();
        HBool(true).to_json(&mut buf).unwrap();
        assert_eq!(buf, "true");

        buf.clear();
        HBool(false).to_json(&mut buf).unwrap();
        assert_eq!(buf, "false");
    }

    #[test]
    fn test_str() {
        let mut buf = String::new();
        HStr::new("hello world".into()).to_json(&mut buf).unwrap();
        assert_eq!(buf, "hello world");
    }

    #[test]
    fn test_number() {
        let mut buf = String::new();
        HNumber::new(42.0, None).to_json(&mut buf).unwrap();
        assert_eq!(buf, "n:42");

        buf.clear();
        HNumber::new(42.0, Some("°F".to_owned().into())).to_json(&mut buf).unwrap();
        assert_eq!(buf, "n:42 °F");
    }

    #[test]
    fn test_ref() {
        let mut buf = String::new();
        HRef::new("site1".into(), None).to_json(&mut buf).unwrap();
        assert_eq!(buf, "r:site1");

        buf.clear();
        HRef::new("site1".into(), Some("Site 1".into())).to_json(&mut buf).unwrap();
        assert_eq!(buf, "r:site1 Site 1");
    }

    #[test]
    fn test_uri() {
        let mut buf = String::new();
        HUri::new("http://example.com").unwrap().to_json(&mut buf).unwrap();
        assert_eq!(buf, "u:http://example.com/");
    }

    #[test]
    fn test_date() {
        let mut buf = String::new();
        HDate::new(2023, 12, 25).to_json(&mut buf).unwrap();
        assert_eq!(buf, "d:2023-12-25");
    }

    #[test]
    fn test_time() {
        let mut buf = String::new();
        HTime::new(14, 30, 0, 0).to_json(&mut buf).unwrap();
        assert_eq!(buf, "h:14:30:00");
    }

    #[test]
    fn test_dict() {
        let mut buf = String::new();
        let mut dict = HDict::<f64>::new();
        dict.set("id".into(), HRef::new("site1".into(), None).to_hbox());
        dict.set("name".into(), HStr::new("Site 1".into()).to_hbox());
        dict.to_json(&mut buf).unwrap();
        assert!(buf.starts_with("{"));
        assert!(buf.ends_with("}"));
        assert!(buf.contains("\"id\""));
        assert!(buf.contains("\"name\""));
    }

    #[test]
    fn test_list() {
        let mut buf = String::new();
        let mut list = HList::new();
        list.push(HStr::new("item1".into()).to_hbox());
        list.push(HNumber::new(42.0, None).to_hbox());
        list.to_json(&mut buf).unwrap();
        
        assert_eq!(buf, "[\"item1\",\"n:42\"]");
        assert!(buf.starts_with("["));
        assert!(buf.ends_with("]"));
    }
}
