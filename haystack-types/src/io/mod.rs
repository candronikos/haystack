use nom::{IResult,Parser};
use nom::bytes::complete::{tag,take,take_while,take_while1};
use nom::branch::alt;
use nom::multi::{separated_list1,many0};
use nom::combinator::{map,opt,verify,recognize,success,value,iterator};
use nom::character::complete::{digit1,char as nom_char,space1};
use nom::character::{is_digit,is_alphanumeric};
use nom::sequence::{tuple,preceded,terminated,separated_pair};
use nom::number::complete::double;
use nom::error::{Error, ErrorKind};

use crate::{HVal, h_bool::HBool, h_null::HNull, h_na::HNA,
    h_marker::HMarker, h_remove::HRemove, h_number::HNumber,
    h_date::HDate, h_datetime::{HDateTime,HOffset}, h_time::HTime,
    h_coord::HCoord, h_str::HStr, h_uri::HUri, h_dict::HDict,
    h_list::HList, h_grid::{HCol,HGrid}};

use crate::common::*;

use std::collections::HashMap;
use core::fmt::Display;
use core::str::FromStr;
use num::Float;

pub mod parse {
use nom::sequence::delimited;
use nom::combinator::map_res;
use super::*;

    macro_rules! into_hval {
        ( $num_type: ty ) => {
            | hval | { Box::new(hval) as Box<dyn HVal> }
        }
    }

    pub mod zinc {
        use super::*;

        pub fn literal<'a,NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&str, Box<dyn HVal>> {
            alt((
                map(null, into_hval!(NumTrait)),
                map(marker, into_hval!(NumTrait)),
                map(remove, into_hval!(NumTrait)),
                map(na, into_hval!(NumTrait)),
                map(boolean, into_hval!(NumTrait)),
                // map(reference, into_hval!(NumTrait)),
                map(string, into_hval!(NumTrait)),
                map(uri, into_hval!(NumTrait)),
                map(number::<NumTrait>, into_hval!(NumTrait)),
                map(datetime, into_hval!(NumTrait)),
                map(date, into_hval!(NumTrait)),
                map(time, into_hval!(NumTrait)),
                map(coord::<NumTrait>, into_hval!(NumTrait)),
                // TODO: Implement tests for collection types
                map(dict::<NumTrait>, into_hval!(NumTrait)),
                map(list::<NumTrait>, into_hval!(NumTrait)),
                map(delimited(tag("<<"),grid::<NumTrait>,tag(">>")), into_hval!(NumTrait)),
                // ref, def, ??symbol?? ARE SYMBOLS VALID IN ZINC?
            )).parse(input)
        }

        pub fn boolean(input: &str) -> IResult<&str, HBool> {
            alt((
                value(HBool(true),tag("T")),
                value(HBool(false),tag("F"))
            ))(input)
        }

        pub fn null(input: &str) -> IResult<&str, HNull> {
            use crate::h_null::NULL;
            map(tag("N"), |_s: &str| { NULL }).parse(input)
        }

        pub fn na(input: &str) -> IResult<&str, HNA> {
            use crate::h_na::NA;
            map(tag("NA"), |_s: &str| { NA }).parse(input)
        }

        pub fn marker(input: &str) -> IResult<&str, HMarker> {
            use crate::h_marker::MARKER;
            map(tag("M"), |_s: &str| { MARKER }).parse(input)
        }

        pub fn remove(input: &str) -> IResult<&str, HRemove> {
            use crate::h_remove::REMOVE;
            map(tag("R"), |_s: &str| { REMOVE }).parse(input)
        }

        pub fn string(input: &str) -> IResult<&str, HStr> {
            let (input,_) = tag("\"")(input)?;
            let mut it = iterator(input, alt((
                value("\x08", tag("\\b")),
                value("\x0C", tag("\\f")),
                value("\n", tag("\\n")),
                value("\r", tag("\\r")),
                value("\t", tag("\\t")),
                value("\"", tag("\\\"")),
                value("\\", tag("\\\\")),
                tag("$"),
                take_while1(unicode_char),
            )));

            let string_literal = it.fold(String::new(),|mut acc, input| { acc.push_str(input); acc });
            let (input,()) = it.finish()?;
            let (input,_) = tag("\"")(input)?;

            Ok((input,HStr(string_literal)))
        }

        pub fn uri(input: &str) -> IResult<&str, HUri> {
            let (input,_) = tag("`")(input)?;
            let mut it = iterator(input, alt((
                value(":", tag("\\:")),
                value("/", tag("\\/")),
                value("#", tag("\\#")),
                value("\"", tag("\\\"")),
                value("[", tag("\\[")),
                value("]", tag("\\]")),
                value("@", tag("\\@")),
                value("`", tag("\\`")),
                value("&", tag("\\&")),
                value("=", tag("\\=")),
                value(";", tag("\\;")),
                value("\\", tag("\\\\")),
                take_while1(unicode_char),
            )));

            let url_literal = it.fold(String::new(),|mut acc, input| { acc.push_str(input); acc });
            let (input,()) = it.finish()?;
            let (input,_) = tag("`")(input)?;

            let uri_res = HUri::new(&url_literal);
            let uri = uri_res.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;
            Ok((input,uri))
        }

        fn get_2_digits(input: &str) -> IResult<&str, &str> {
            verify(take(2usize),|s: &str| s.chars().all(|c| char::is_digit(c,10)))(input)
        }

        fn get_offset(input: &str) -> IResult<&str, (i32, u32, u32)> {
            tuple((
                alt((value(-1,nom_char('-')),value(1,nom_char('+')))),
                map(get_2_digits, |s| u32::from_str_radix(s,10).unwrap()),
                preceded(tag(":"), map(get_2_digits, |s| u32::from_str_radix(s,10).unwrap()))
            ))(input)
        }

        fn get_named_tz(input: &str) -> IResult<&str, &str> {
            take_while(|c: char| is_alphanumeric(c as u8) || c == '/' || c== '-' || c== '_' || c== '+')(input)
        }

        fn tz(input: &str) -> IResult<&str, (chrono_tz::Tz, HOffset)> {
            use chrono_tz::{Tz, UTC};
            use chrono::offset::FixedOffset;

            let (input, (first, second)) = alt((
                tuple((recognize(get_offset), preceded(tag(" "), get_named_tz))),
                tuple((tag("Z"), preceded(tag(" "), get_named_tz))),
                tuple((tag("Z"), success(""))),
            ))(input)?;

            let timezone: (chrono_tz::Tz, HOffset) = match first {
                "Z" => match second {
                    "" => (UTC, HOffset::Utc),
                    _ => {
                        let t = Tz::from_str(second).unwrap();
                        (t, HOffset::Variable(chrono::Duration::minutes(1 * 60 + 1)))
                    }
                },
                _ => {
                    let (_,(sign,hours,minutes)) = get_offset(first)?;
                    (Tz::from_str(second).unwrap(), HOffset::Fixed(FixedOffset::east(sign * (hours as i32 * 3600 + minutes as i32 * 60))))
                }
            };

            Ok((input,timezone))
        }

        pub fn datetime(input: &str) -> IResult<&str, HDateTime> {
            let start = input;
            let (input, (yr, mo, day, hr, min, sec, nano, tz)) = tuple((
                terminated(map(take(4usize), |s| i32::from_str_radix(s,10).unwrap()),tag("-")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10).unwrap()),tag("-")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10).unwrap()),tag("T")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10).unwrap()),tag(":")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10).unwrap()),tag(":")),
                map(take(2usize), |s| u32::from_str_radix(s,10).unwrap()),
                opt(preceded(tag("."),map(digit1, |s| u32::from_str_radix(s,10).unwrap()))),
                tz
            ))(start)?;

            Ok((input, HDateTime::new(
                yr, mo, day, hr, min, sec,
                if let Some(nano) = nano { nano } else { 0 },
                tz
            )))
        }

        fn coord_deg<T: Float + Display + FromStr>(input: &str) -> IResult<&str, T> {
            map_res(recognize(tuple((opt(tag("-")),digit1,opt(tuple((tag("."),digit1)))))),|s: &str| s.parse::<T>())(input)
        }

        pub fn coord<T: Float + Display + FromStr>(input: &str) -> IResult<&str, HCoord<T>> {
            let (input,(lat,long)) = tuple((delimited(tag("C("),coord_deg,tag(",")),terminated(coord_deg,tag(")"))))(input)?;
    
            Ok((input,HCoord::new(lat,long)))
        }

        fn tags<'a,NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&str,Option<HashMap<String,Box<dyn HVal + 'a>>>> {
            let (input,res) = opt(separated_list1(tag("\x20"),alt((
                map(id,|i| (i.to_owned(),Box::new(crate::h_marker::MARKER) as Box<dyn HVal>)),
                map(separated_pair(id,tag(":"),literal::<NumTrait>), |(i,v)| (i.to_owned(),v)),
            ))))(input)?;

            let res = match res {
                Some(v) => {
                    let mut map: HashMap<String, Box<dyn HVal>> = HashMap::new();
                    v.into_iter().for_each(|(k,v)| { map.insert(k.to_owned(), v).unwrap(); () });
                    Some(map)
                },
                None => None
            };

            Ok((input,res))
        }

        fn tags_list<'a,NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&'a str,Option<Vec<Box<dyn HVal>>>> {
            let (input,res) = opt(separated_list1(tag(","),literal::<NumTrait>))(input)?;

            Ok((input,res))
        }

        pub fn dict<'a, NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&str, HDict> {
            let (input,opt_dict) = delimited(tag("{"),tags::<NumTrait>,tag("}"))(input)?;

            let dict = match opt_dict {
                Some(dict) => dict,
                None => HashMap::new()
            };

            Ok((input,HDict::from_map(dict)))
        }

        pub fn list<NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&str, HList> {
            let (input,opt_vec) = delimited(tag("["),tags_list::<NumTrait>,tag("]"))(input)?;

            let vec = match opt_vec {
                Some(vec) => vec,
                None => Vec::new()
            };

            Ok((input,HList::from_vec(vec)))
        }

        pub fn grid_meta<NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&str, HashMap<String,Box<dyn HVal>>> {
            let (input,opt_dict) = tags::<NumTrait>(input)?;

            let dict = match opt_dict {
                Some(dict) => dict,
                None => HashMap::new()
            };

            Ok((input,dict))
        }

        pub fn cols<NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&str, Vec<HCol>> {
            let (input,columns) = separated_list1(tag(","), separated_pair(id, space1, tags::<NumTrait>))(input)?;
            let columns = columns.into_iter().map(|(id,meta)| HCol::new(id.to_string(),meta));
            Ok((input,columns.collect()))
        }

        pub fn grid<NumTrait: 'static + Float + Display + FromStr>(input: &'static str) -> IResult<&str, HGrid> {
            let (input,version) = delimited(tag("ver:"), recognize(double), space1)(input)?;

            // Grid Meta
            let (input,meta) = terminated(grid_meta::<NumTrait>, tag("\n"))(input)?;

            // Cols
            let (input,columns) = terminated(cols::<NumTrait>, tag("\n"))(input)?;

            // Rows
            let (input,rows) = many0(separated_list1(tag(","),opt(literal::<NumTrait>)))(input)?;

            let grid = HGrid::from_row_vec(columns,rows)
                .add_meta(meta).unwrap();

            Ok((input,grid))
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn parse_unicode_char() {
                let hello: IResult<&str,&str> = take_while(unicode_char)("Hello\n\r\t\"\\");
                assert_eq!(hello,Ok(("\n\r\t\"\\","Hello")));
            }

            #[test]
            fn parse_string_02() {
                assert_eq!(string("\"He\\tllo\""),Ok(("",HStr("He\tllo".to_owned()))));
            }

            #[test]
            fn parse_bool() {
                assert_eq!(boolean("T").unwrap(),("",HBool(true)));
                assert_eq!(boolean("F").unwrap(),("",HBool(false)),);
            }

            #[test]
            fn parse_null() {
                assert_eq!(null("N").unwrap(),("",crate::h_null::NULL));
            }

            #[test]
            fn parse_na() {
                assert_eq!(na("NA").unwrap(),("",crate::h_na::NA));
            }

            #[test]
            fn parse_marker() {
                assert_eq!(marker("M").unwrap(),("",crate::h_marker::MARKER));
            }

            #[test]
            fn parse_remove() {
                assert_eq!(remove("R").unwrap(),("",crate::h_remove::REMOVE));
            }

            #[test]
            fn parse_datetime() {
                let tz_obj = (chrono_tz::Tz::from_str("America/Los_Angeles").unwrap(), HOffset::Fixed(chrono::offset::FixedOffset::east(-1 * 8 * 3600)));
                assert_eq!(datetime("2010-11-28T07:23:02.773-08:00 America/Los_Angeles").unwrap(),("",HDateTime::new(2010,11,28,7,23,2,773,tz_obj)));
            }

            #[test]
            fn parse_coord() {
                assert_eq!(coord("C(1.5,-9)").unwrap(),("",HCoord::new(1.5,-9f64)));
            }
        }
    }

    pub fn is_digits(chr: char) -> bool {
        is_digit(chr as u8) && chr=='_'
    }

    pub fn digits(input: &str) -> IResult<&str, (&str, &str)> {
        use nom::branch::permutation;

        permutation((digit1, take_while(is_digits))).parse(input)
    }

    pub fn exp(input: &str) -> IResult<&str, (bool, (&str, &str))> {
        use nom::sequence::pair;

        preceded(alt((tag("e"),tag("E"))),pair(
            opt(alt((tag("+"),tag("-"))))
                .map(|sign| if let Some(c) = sign { c!="-" } else { true } ),
            digits
        ))(input)
    }

    pub fn number<'a, T: 'a + Float + Display + FromStr>(start: &str) -> IResult<&str, HNumber<T>> {
        use std::slice;

        let (input, is_positive) = map(opt(tag("-")),|d| d.is_none())(start)?;
        let (input, integer) = digits(input)?;
        let (input, decimals) = opt(preceded(tag("."),digits))(input)?;
        let (input, exponent) = opt(exp)(input)?;
        let number_slice = unsafe {
            slice::from_raw_parts(start.as_ptr(),
            (input.as_ptr() as usize) - (start.as_ptr() as usize))
        };

        let number_ty: T;

        if let Ok(number_str) = std::str::from_utf8(number_slice) {
            let number_res = number_str.parse::<T>();
            if let Ok(number_ty_ok) = number_res {
                number_ty = number_ty_ok;
            } else {
                return Err(nom::Err::Error(nom::error::Error{ input: number_str, code: nom::error::ErrorKind::Float }));
            }
        } else {
            // TODO: Handle numbers with '_' in the digits
            return Err(nom::Err::Error(nom::error::Error{
                input: unsafe { std::str::from_utf8_unchecked(number_slice) },
                code: nom::error::ErrorKind::Float }));
        }

        Ok((input, HNumber::new(number_ty, None)))
        // TODO: Extract units
    }

    pub fn date(input: &str) -> IResult<&str, HDate> {
        let (input, year) = map(take(4usize), |s| i32::from_str_radix(s, 10) ).parse(input)?;
        let year = year.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;

        let (input, _) = tag("-")(input)?;
        let (input, month) = map(take(2usize), |s| u32::from_str_radix(s, 10) ).parse(input)?;
        let month = month.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;

        let (input, _) = tag("-")(input)?;
        let (input, day) = map(take(2usize), |s| u32::from_str_radix(s, 10) ).parse(input)?;
        let day = day.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;

        Ok((input,HDate::new(year, month, day)))
    }

    pub fn time(input: &str) -> IResult<&str, HTime> {
        let (input, hour) = map(take(2usize), |s| {u32::from_str_radix(s, 10)}).parse(input)?;
        let hour = hour.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;

        let (input, _) = tag(":")(input)?;
        let (input, min) = map(take(2usize), |s| {u32::from_str_radix(s, 10)}).parse(input)?;
        let min = min.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;

        let (input, _) = tag(":")(input)?;
        let (input, sec) = map(take(2usize), |s| {u32::from_str_radix(s, 10)}).parse(input)?;
        let sec = sec.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;

        let (input, _) = tag(".")(input)?;
        let (input, nano) = map(opt(digit1), |s| {if let Some(s) = s {u32::from_str_radix(s, 10)} else {Ok(0)}}).parse(input)?;
        let nano = nano.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;

        Ok((input, HTime::new(hour, min, sec, nano)))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_date() {
            assert_eq!(date("2010-11-28").unwrap(),("",HDate::new(2010,11,28)));
        }

        #[test]
        fn parse_time() {
            assert_eq!(time("07:23:02.773").unwrap(),("",HTime::new(07,23,02,773)));
        }
    }
}