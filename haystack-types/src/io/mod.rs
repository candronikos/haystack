use nom::{IResult,Parser};
use nom::bytes::complete::{tag,take,take_while,take_while1};
use nom::branch::alt;
use nom::multi::separated_list1;
use nom::combinator::{map,opt,verify,recognize,success,value,iterator};
use nom::character::complete::{digit1,char as nom_char,space1};
use nom::character::{is_digit,is_alphanumeric};
use nom::sequence::{preceded,terminated,separated_pair};
use nom::number::complete::double;
use nom::error::{Error, ErrorKind};

use crate::{HVal, h_bool::HBool, h_null::HNull, h_na::HNA, h_marker::HMarker,
    h_remove::HRemove, h_number::{HNumber,HUnit}, h_ref::HRef,
    h_date::HDate, h_datetime::{HDateTime,HOffset}, h_time::HTime,
    h_coord::HCoord, h_str::HStr, h_uri::HUri, h_dict::HDict,
    h_list::HList, h_grid::HGrid};

use crate::common::*;

use std::collections::HashMap;
use core::fmt::Display;
use core::str::FromStr;
use num::Float;

pub mod parse {
use nom::sequence::delimited;
use nom::combinator::map_res;
use super::*;

    macro_rules! into_box {
        ( $fn: expr, $num_type: ty, $lt: lifetime ) => {
            map($fn,| hval | { Box::new(hval) as Box<dyn HVal<$lt,$num_type> + $lt> })
        }
    }

    pub mod zinc {
        use nom::{character::complete::newline, combinator::all_consuming, error::ParseError, AsChar, Err};

        use super::*;

        pub fn literal<'a, 'b, NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str, Box<dyn HVal<'a,NumTrait> + 'a>> {
            alt((
                into_box!(na,NumTrait,'a),
                into_box!(null,NumTrait,'a),
                into_box!(marker,NumTrait,'a),
                into_box!(remove,NumTrait,'a),
                into_box!(boolean,NumTrait,'a),
                into_box!(reference, NumTrait,'a),
                into_box!(string,NumTrait,'a),
                into_box!(uri,NumTrait,'a),
                into_box!(datetime,NumTrait,'a),
                into_box!(date,NumTrait,'a),
                into_box!(time,NumTrait,'a),
                into_box!(number,NumTrait,'a),
                into_box!(coord,NumTrait,'a),
                // TODO: Implement tests for collection types
                into_box!(dict,NumTrait,'a),
                into_box!(list,NumTrait,'a),
                into_box!(delimited(tag("<<"),grid::<NumTrait>,tag(">>")),NumTrait,'a),
                // TODO: Implement symbol type
            )).parse(input)
        }

        pub fn boolean(input: &str) -> IResult<&str, HBool> {
            alt((
                value(HBool(true),tag("T")),
                value(HBool(false),tag("F"))
            )).parse(input)
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
                value("$", tag("\\$")),
                take_while1(unicode_char('"')),
            )));

            let string_literal = it.by_ref().fold(String::new(),|mut acc, input| { acc.push_str(input); acc });
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
                take_while1(unicode_char('`')),
            )));

            let url_literal = it.by_ref().fold(String::new(),|mut acc, input| { acc.push_str(input); acc });
            let (input,()) = it.finish()?;
            let (input,_) = tag("`")(input)?;

            let uri_res = HUri::new(&url_literal);
            let uri = uri_res.or(Err(nom::Err::Error(Error{ input: input, code: ErrorKind::Digit })))?;
            Ok((input,uri))
        }

        pub fn ref_chars_body<I>(prefix:char) -> impl FnMut(I) -> IResult<I,I>
        where
            I: nom::Input + Clone,
            <I as nom::Input>::Item: nom::AsChar + Copy,
        {
            
        move |input| preceded(nom::character::complete::char::<I, nom::error::Error<I>>(prefix), take_while1(|c: <I as nom::Input>::Item| {
            let c = c.as_char();
            c.is_ascii_alphanumeric() || match c {
                '_' | ':' | '-' | '.' | '~' => true,
                _ => false
            }
        })).parse(input)
        }

        fn reference(input: &str) -> IResult<&str, HRef> {
            let (input,(ref_str,dis_str)) = ((
                ref_chars_body('@'),
                opt(preceded(tag(" "), string)),
            )).parse(input)?;
            Ok((input,HRef::new(ref_str.to_owned(), dis_str.map(|s| s.into_string()))))
        }

        fn get_2_digits(input: &str) -> IResult<&str, &str> {
            verify(take(2usize),|s: &str| s.chars().all(|c| char::is_digit(c,10))).parse(input)
        }

        fn get_offset(input: &str) -> IResult<&str, (i32, u32, u32)> {
            ((
                alt((value(-1,nom_char('-')),value(1,nom_char('+')))),
                map(get_2_digits, |s| u32::from_str_radix(s,10).unwrap()),
                preceded(tag(":"), map(get_2_digits, |s| u32::from_str_radix(s,10).unwrap()))
            )).parse(input)
        }

        fn get_named_tz(input: &str) -> IResult<&str, &str> {
            recognize(((
                take_while1(|c: char| c.is_ascii_uppercase()),
                take_while(|c: char| is_alphanumeric(c as u8) || c == '/' || c== '-' || c== '_' || c== '+')
            ))).parse(input)
        }

        fn timezone(input: &str) -> IResult<&str, (String, HOffset)> {
            use chrono_tz::{Tz, UTC};
            use chrono::offset::FixedOffset;

            let (input, (first, second)) = alt((
                (recognize(get_offset), preceded(tag(" "), get_named_tz)),
                (tag("Z"), preceded(tag(" "), get_named_tz)),
                (tag("Z"), success("")),
            )).parse(input)?;

            // TODO: Implement with TZ instead of String
            let timezone: (String, HOffset) = match first {
                "Z" => match second {
                    // "" => (UTC, HOffset::Utc),
                    "" => ("UTC".to_owned(), HOffset::Utc),
                    _ => {
                        // let t = Tz::from_str(second).unwrap();
                        // (t, HOffset::Variable(chrono::Duration::minutes(1 * 60 + 1)))
                        (second.to_owned(), HOffset::Variable(chrono::Duration::minutes(1 * 60 + 1)))
                    }
                },
                _ => {
                    let (_,(sign,hours,minutes)) = get_offset(first)?;
                    // (Tz::from_str(second).unwrap(), HOffset::Fixed(FixedOffset::east(sign * (hours as i32 * 3600 + minutes as i32 * 60))))
                    (second.to_owned(), HOffset::Fixed(FixedOffset::east(sign * (hours as i32 * 3600 + minutes as i32 * 60))))
                }
            };

            Ok((input,timezone))
        }

        pub fn datetime(input: &str) -> IResult<&str, HDateTime> {
            let start = input;
            let (input, (yr, mo, day, hr, min, sec, nano, tz)) = (
                terminated(map(take(4usize), |s| i32::from_str_radix(s,10)),tag("-")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10)),tag("-")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10)),tag("T")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10)),tag(":")),
                terminated(map(take(2usize), |s| u32::from_str_radix(s,10)),tag(":")),
                map(take(2usize), |s| u32::from_str_radix(s,10)),
                opt(preceded(tag("."),map(digit1, |s| u32::from_str_radix(s,10)))),
                timezone
            ).parse(start)?;

            let yr = yr.or(Err(nom::Err::Error(Error{ input: start, code: ErrorKind::Digit })))?;
            let mo = mo.or(Err(nom::Err::Error(Error{ input: start, code: ErrorKind::Digit })))?;
            let day = day.or(Err(nom::Err::Error(Error{ input: start, code: ErrorKind::Digit })))?;
            let hr = hr.or(Err(nom::Err::Error(Error{ input: start, code: ErrorKind::Digit })))?;
            let min = min.or(Err(nom::Err::Error(Error{ input: start, code: ErrorKind::Digit })))?;
            let sec = sec.or(Err(nom::Err::Error(Error{ input: start, code: ErrorKind::Digit })))?;

            Ok((input, HDateTime::new(
                yr, mo, day, hr, min, sec,
                if let Some(nano) = nano {
                    nano.or(Err(nom::Err::Error(Error{ input: start, code: ErrorKind::Digit })))?
                } else { 0 },
                tz
            )))
        }

        fn coord_deg<T: Float + Display + FromStr>(input: &str) -> IResult<&str, T> {
            map_res(recognize(((opt(tag("-")),digit1,opt(((tag("."),digit1)))))),|s: &str| s.parse::<T>()).parse(input)
        }

        pub fn coord<T: Float + Display + FromStr>(input: &str) -> IResult<&str, HCoord<T>> {
            let (input,(lat,long)) = (delimited(tag("C("),coord_deg,tag(",")),terminated(coord_deg,tag(")"))).parse(input)?;
    
            Ok((input,HCoord::new(lat,long)))
        }

        fn tags<'a,'b,NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str,Option<HashMap<String,Box<dyn HVal<'a,NumTrait> + 'a>>>> {
            let (input,res_opt) = opt(separated_list1(
                tag(" "),
                ((id, opt(preceded(tag(":"), literal::<NumTrait>))))
            )).parse(input)?;

            let mut map: HashMap<String, Box<dyn HVal<'a,NumTrait> + 'a>> = HashMap::new();
            let map_opt: Option<_>;
            if let Some(res) = res_opt {
                res.into_iter().for_each(|(k,v)| {
                    map.insert(k.to_owned(), v.unwrap_or(Box::new(crate::h_marker::MARKER) as Box<dyn HVal<'a,NumTrait>>));
                });
                map_opt = Some(map);
            } else {
                map_opt = None;
            }

            Ok((input,map_opt))
        }

        fn tags_list<'a,'b,NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str,Option<Vec<Box<dyn HVal<'a,NumTrait> + 'a>>>> {
            let (input,res) = opt(separated_list1(tag(","),literal::<NumTrait>)).parse(input)?;

            Ok((input,res))
        }

        pub fn dict<'a, 'b, NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str, HDict<'a,NumTrait>> {
            let (input,opt_dict) = delimited(tag("{"),tags::<NumTrait>,tag("}")).parse(input)?;

            let dict = match opt_dict {
                Some(dict) => dict,
                None => HashMap::new()
            };

            Ok((input,HDict::from_map(dict)))
        }

        pub fn list<'a, 'b, NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str, HList<'a,NumTrait>> {
            let (input,opt_vec) = delimited(tag("["),tags_list::<NumTrait>,tag("]")).parse(input)?;

            let vec = match opt_vec {
                Some(vec) => vec,
                None => Vec::new()
            };

            Ok((input,HList::from_vec(vec)))
        }

        pub fn grid_meta<'a, 'b, NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str, HashMap<String,Box<dyn HVal<'a,NumTrait> + 'a>>> {
            let (input,opt_dict) = tags::<NumTrait>(input)?;

            let dict = match opt_dict {
                Some(dict) => dict,
                None => HashMap::new()
            };

            Ok((input,dict))
        }

        pub fn cols<'a, 'b, NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str, Vec<(String,Option<HashMap<String,Box<dyn HVal<'a,NumTrait> + 'a>>>)>> {
            let (input,columns): (_,Vec<(_, Option<HashMap<_,Box<dyn HVal<'a,NumTrait>>>>)>) = separated_list1(tag(","), separated_pair(id, space1, tags::<NumTrait>)).parse(input)?;
            let columns = columns.into_iter().map(|(id,meta)| (id.to_owned(),meta));
            let columns = columns.collect();
            Ok((input,columns))
        }

        pub fn grid_err<NumTrait: Float + Display + FromStr>(input: &str) -> IResult<&str, HGrid<NumTrait>> {
            let (input,_) = tag("ver:\"3.0\"")(input)?;
            let (input,meta) = delimited(space1, grid_meta::<NumTrait>, tag("\n")).parse(input)?;
            let (_, is_empty) = all_consuming(map(terminated(tag("empty"), take_while1(|c| c == '\n')), |_| true)).parse(input)?;
            if is_empty || meta.contains_key("err") {
                //let dis = meta.get("dis").unwrap().get_string_val().unwrap().into_string();
                let dis = match meta.get("dis") {
                    Some(v) => {
                        // v.get_string_val().unwrap().into_string()
                        if let Some(s) = v.get_string_val() {
                            s.clone_into_string()
                        } else {
                            return Err(nom::Err::Error(Error{
                                input: input,
                                code: ErrorKind::Tag
                            }))
                        }
                    },
                    None => return Err(nom::Err::Error(Error{
                        input: input,
                        code: ErrorKind::Tag
                    }))
                };
                
                // meta.get("errTrace").map(|s| s.get_string_val().unwrap().into_string())
                let errTrace = match meta.get("errTrace") {
                    Some(v) => {
                        // v.get_string_val().unwrap().into_string()
                        if let Some(s) = v.get_string_val() {
                            Some(s.clone_into_string())
                        } else {
                            return Err(nom::Err::Error(Error{
                                input: input,
                                code: ErrorKind::Tag
                            }))
                        }
                    },
                    None => return Err(nom::Err::Error(Error{
                        input: input,
                        code: ErrorKind::Tag
                    }))
                };

                return Ok((input, HGrid::Error {
                    dis: dis,
                    errTrace: errTrace,
                }));
                
            }

            Err(nom::Err::Error(Error{
                input: input,
                code: ErrorKind::Tag
            }))
        }

        pub fn grid<'a, 'b, NumTrait: 'a + Float + Display + FromStr>(input: &'b str) -> IResult<&'b str, HGrid<'a,NumTrait>> {
            let (input,version) = delimited(tag("ver:\""), recognize(double), tag("\"")).parse(input)?;

            // Grid Meta
            let (input,meta) = delimited(space1, grid_meta::<NumTrait>, tag("\n")).parse(input)?;

            let (_, is_empty) = all_consuming(map(terminated(tag("\nempty"), take_while1(|c| c == '\n')), |_| true)).parse(input)?;
            if is_empty {
                return Ok((input, HGrid::Empty));
                
            }
            
            // Cols
            let (input,columns) = terminated(cols::<NumTrait>, tag("\n")).parse(input)?;

            // Rows
            let row_width = columns.len();
            let (input,rows) = separated_list1(tag("\n"),
                verify(separated_list1(tag(","),opt(literal::<NumTrait>)),|v: &Vec<Option<Box<dyn HVal<NumTrait>>>>| v.len()==row_width)).parse(input)?;

            let grid = HGrid::from_row_vec(columns,rows)
                .add_meta(meta).unwrap();

            Ok((input,grid))
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::HCast;

            #[test]
            fn parse_unicode_char() {
                let hello: IResult<&str,&str> = take_while(unicode_char('"'))("Hello\n\r\t\"\\");
                assert_eq!(hello,Ok(("\n\r\t\"\\","Hello")));
            }

            #[test]
            fn parse_tags() {
                let input = "dis:\"Fri 31-Jul-2020\" view:\"chart\" title:\"Line\" chartNoScroll chartLegend:\"hide\" hisStart:2020-07-31T00:00:00-04:00 New_York hisEnd:2020-08-01T00:00:00-04:00 New_York hisLimit:10000";

                let res = tags::<f64>(input);
                if let Ok(e) = res {
                    let mut buf = String::new();
                    let temp1 = &e.1.unwrap();
                    let v = temp1.get("dis").unwrap();
                    v.to_zinc(&mut buf).unwrap();
                    let rhs = Box::new(HStr("Fri 31-Jul-2020".to_owned())) as Box<dyn HVal<f64>>;
                    assert_eq!(v,&rhs)
                } else {
                    panic!("Failed to parse separated list")
                }
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
                // TODO: Implement with Haystack Timezones so they're valid with the `chrono` library
                // let tz_obj = (chrono_tz::Tz::from_str("America/Los_Angeles").unwrap(), HOffset::Fixed(chrono::offset::FixedOffset::east(-1 * 8 * 3600)));
                let tz_obj = ("America/Los_Angeles".to_owned(), HOffset::Fixed(chrono::offset::FixedOffset::east(-1 * 8 * 3600)));
                assert_eq!(datetime("2010-11-28T07:23:02.773-08:00 America/Los_Angeles").unwrap(),("",HDateTime::new(2010,11,28,7,23,2,773,tz_obj)));
            }

            #[test]
            fn parse_coord() {
                assert_eq!(coord("C(1.5,-9)").unwrap(),("",HCoord::new(1.5,-9f64)));
            }

            #[test]
            fn coerce_na2hval() {
                use crate::h_na::NA;
                let v = literal::<f64>("NA").unwrap();
                let lhs = v.1.get_na();
                assert_eq!(lhs,Some(&NA))
            }

            macro_rules! assert_literal {
                ( $val: literal, $get: ident, $rhs: expr ) => {
                    let v = literal::<f64>($val).unwrap();
                    let lhs = v.1.$get();
                    assert_eq!(lhs,Some(&$rhs));
                }
            }

            #[test]
            fn coerce_hval() {
                use crate::h_null::NULL;
                use crate::h_marker::MARKER;
                use crate::h_remove::REMOVE;
                use crate::h_bool::HBool;
                use crate::h_na::NA;
                use crate::h_str::HStr;
                use crate::h_uri::HUri;

                assert_literal!("N",get_null,NULL);
                assert_literal!("M",get_marker,MARKER);
                assert_literal!("R",get_remove,REMOVE);
                assert_literal!("T",get_bool,HBool(true));
                assert_literal!("F",get_bool,HBool(false));
                assert_literal!("NA",get_na,NA);
                assert_literal!(r#""Hello\nSmidgen\"""#,get_string,HStr("Hello\nSmidgen\"".to_owned()));
                assert_literal!("`http://www.google.com`",get_uri,HUri::new("http://www.google.com").unwrap());
                assert_literal!("1.5kWh",get_number,HNumber::new(1.5f64,Some(HUnit::new("kWh".to_owned()))));
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
        )).parse(input)
    }

    pub fn is_unit(c: char) -> bool {
        c.is_ascii_alphabetic() || c>=(128 as char) || match c {
            '%' | '_' | '/' | '$' => true,
            _ => false
        }
    }

    pub fn unit(input: &str) -> IResult<&str, HUnit> {
        let (input,unit_str) = take_while1(is_unit)(input)?;
        Ok((input,HUnit::new(unit_str.to_owned())))
    }

    pub fn number<'a, T: 'a + Float + Display + FromStr>(input: &str) -> IResult<&str, HNumber<T>> {
        use std::slice;

        let start = input;
        let (input, is_positive) = map(opt(tag("-")),|d| d.is_none()).parse(start)?;
        let (input, integer) = digits(input)?;
        let (input, decimals) = opt(preceded(tag("."),digits)).parse(input)?;
        let (input, exponent) = opt(exp).parse(input)?;
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

        let (input, unit_opt) = opt(unit).parse(input)?;

        Ok((input, HNumber::new(number_ty, unit_opt)))
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
        fn parse_id() {
            assert_eq!(id("asdasd1223_").unwrap(),("","asdasd1223_"));
        }

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