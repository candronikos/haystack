use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while1};
use nom::character::complete::{char as nom_char, digit1, space1};
use nom::character::{is_alphanumeric, is_digit};
use nom::combinator::{iterator, map, opt, recognize, success, value, verify};
use nom::error::{Error, ErrorKind};
use nom::multi::separated_list1;
use nom::number::complete::double;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{IResult, Parser};

use core::fmt::Display;
use core::str::FromStr;
use num::Float;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

pub mod write;

use crate::{
    HVal,
    h_bool::HBool,
    h_coord::HCoord,
    h_date::HDate,
    h_datetime::{HDateTime, HTimezone},
    h_dict::HDict,
    h_grid::HGrid,
    h_list::HList,
    h_marker::HMarker,
    h_na::HNA,
    h_null::HNull,
    h_number::{HNumber, HUnit, NumTrait},
    h_ref::HRef,
    h_remove::HRemove,
    h_str::HStr,
    h_symbol::HSymbol,
    h_time::HTime,
    h_uri::HUri,
    h_val::HBox,
    h_xstr::HXStr,
};

use crate::common::*;

pub struct ParseHint {
    tz: OnceLock<chrono_tz::Tz>,
}

impl ParseHint {
    fn tz(&mut self) -> &mut OnceLock<chrono_tz::Tz> {
        &mut self.tz
    }
}

impl Default for ParseHint {
    fn default() -> Self {
        ParseHint {
            tz: OnceLock::new(),
        }
    }
}

fn get_timezone(tz_name: &str, dt_cell: &mut ParseHint) -> Result<chrono_tz::Tz, &'static str> {
    let _tz = dt_cell.tz().get_or_init(|| chrono_tz::Tz::UTC);
    match dt_cell.tz().get_mut() {
        Some(tz) => {
            if tz.name() == tz_name || tz.name().split('/').last() == Some(tz_name) {
                return Ok(tz.clone());
            }
            let tz_clone = match chrono_tz::Tz::from_str(tz_name) {
                Ok(tz_clone) => tz_clone,
                Err(_) => chrono_tz::TZ_VARIANTS
                    .iter()
                    .find(|&&t| t.name() == tz_name || t.name().split('/').last() == Some(tz_name))
                    .ok_or_else(|| "Invalid timezone name")?
                    .clone(),
            };

            *tz = tz_clone.clone();
            return Ok(tz_clone);
        }
        None => panic!("Timezone cell is not initialized. This should never happen."),
    }
}

pub mod parse {
    use super::*;
    use nom::combinator::map_res;
    use nom::sequence::delimited;

    macro_rules! into_box {
        ( $fn: expr, $num_type: ty, $lt: lifetime ) => {
            map($fn, |hval| Rc::new(hval) as HBox<$lt, $num_type>)
        };
    }

    pub mod zinc {
        use chrono::FixedOffset;
        use chrono_tz::Tz;
        use nom::{
            AsChar, Err, character::complete::newline, combinator::all_consuming, error::ParseError,
        };

        use super::*;

        pub fn literal<'out, T: NumTrait + 'out>(
            dt_cell: &mut ParseHint,
        ) -> impl FnMut(&str) -> IResult<&str, HBox<'out, T>> {
            |input: &str| {
                alt((
                    into_box!(na,T,'out),
                    into_box!(null,T,'out),
                    into_box!(marker,T,'out),
                    into_box!(remove,T,'out),
                    into_box!(boolean,T,'out),
                    into_box!(reference, T,'out),
                    into_box!(symbol, T,'out),
                    into_box!(string,T,'out),
                    into_box!(xstring,T,'out),
                    into_box!(uri,T,'out),
                    into_box!(datetime(dt_cell),T,'out),
                    into_box!(date,T,'out),
                    into_box!(time,T,'out),
                    into_box!(number::<T>,T,'out),
                    into_box!(coord::<T>,T,'out),
                    // TODO: Implement tests for collection types
                    into_box!(dict::<T>,T,'out),
                    into_box!(list::<T>,T,'out),
                    into_box!(delimited(tag("<<"),grid::<T>,tag(">>")),T,'out),
                ))
                .parse(input)
            }
        }

        pub fn boolean(input: &str) -> IResult<&str, HBool> {
            alt((value(HBool(true), tag("T")), value(HBool(false), tag("F")))).parse(input)
        }

        pub fn null(input: &str) -> IResult<&str, HNull> {
            use crate::h_null::NULL;
            map(tag("N"), |_s: &str| NULL).parse(input)
        }

        pub fn na(input: &str) -> IResult<&str, HNA> {
            use crate::h_na::NA;
            map(tag("NA"), |_s: &str| NA).parse(input)
        }

        pub fn marker(input: &str) -> IResult<&str, HMarker> {
            use crate::h_marker::MARKER;
            map(tag("M"), |_s: &str| MARKER).parse(input)
        }

        pub fn remove(input: &str) -> IResult<&str, HRemove> {
            use crate::h_remove::REMOVE;
            map(tag("R"), |_s: &str| REMOVE).parse(input)
        }

        pub fn string(input: &str) -> IResult<&str, HStr> {
            let (input, _) = tag("\"")(input)?;
            let mut it = iterator(
                input,
                alt((
                    value("\x08", tag("\\b")),
                    value("\x0C", tag("\\f")),
                    value("\n", tag("\\n")),
                    value("\r", tag("\\r")),
                    value("\t", tag("\\t")),
                    value("\"", tag("\\\"")),
                    value("\\", tag("\\\\")),
                    value("$", tag("\\$")),
                    take_while1(unicode_char('"')),
                )),
            );

            let string_literal = it.by_ref().fold(String::new(), |mut acc, input| {
                acc.push_str(input);
                acc
            });
            let (input, ()) = it.finish()?;
            let (input, _) = tag("\"")(input)?;

            Ok((input, HStr(string_literal)))
        }

        pub fn xstring(input: &str) -> IResult<&str, HXStr> {
            let (input, _x_type) = recognize((
                take_while1(|c: char| c.is_ascii_uppercase()),
                take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
            ))
            .parse(input)?;

            let (input, x_val) = delimited(tag(")"), string, tag(")")).parse(input)?;

            Ok((input, HXStr::new(_x_type.to_owned(), x_val.into_string())))
        }

        pub fn uri(input: &str) -> IResult<&str, HUri> {
            let (input, _) = tag("`")(input)?;
            let mut it = iterator(
                input,
                alt((
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
                )),
            );

            let url_literal = it.by_ref().fold(String::new(), |mut acc, input| {
                acc.push_str(input);
                acc
            });
            let (input, ()) = it.finish()?;
            let (input, _) = tag("`")(input)?;

            let uri_res = HUri::new(&url_literal);
            let uri = uri_res.or(Err(nom::Err::Error(Error {
                input: input,
                code: ErrorKind::Digit,
            })))?;
            Ok((input, uri))
        }

        pub fn ref_chars_body<I>(prefix: char) -> impl FnMut(I) -> IResult<I, I>
        where
            I: nom::Input + Clone,
            <I as nom::Input>::Item: nom::AsChar + Copy,
        {
            move |input| {
                preceded(
                    nom::character::complete::char::<I, nom::error::Error<I>>(prefix),
                    take_while1(|c: <I as nom::Input>::Item| {
                        let c = c.as_char();
                        c.is_ascii_alphanumeric()
                            || match c {
                                '_' | ':' | '-' | '.' | '~' => true,
                                _ => false,
                            }
                    }),
                )
                .parse(input)
            }
        }

        pub fn reference(input: &str) -> IResult<&str, HRef> {
            let (input, (ref_str, dis_str)) =
                ((ref_chars_body('@'), opt(preceded(tag(" "), string)))).parse(input)?;
            Ok((
                input,
                HRef::new(ref_str.to_owned(), dis_str.map(|s| s.into_string())),
            ))
        }

        pub fn symbol(input: &str) -> IResult<&str, HSymbol> {
            let (input, symbol_str) = ref_chars_body('^')(input)?;
            Ok((input, HSymbol::new(symbol_str.to_owned())))
        }

        fn get_2_digits(input: &str) -> IResult<&str, &str> {
            verify(take(2usize), |s: &str| {
                s.chars().all(|c| char::is_digit(c, 10))
            })
            .parse(input)
        }

        fn get_offset(input: &str) -> IResult<&str, (i32, u32, u32)> {
            ((
                alt((value(-1, nom_char('-')), value(1, nom_char('+')))),
                map(get_2_digits, |s| u32::from_str_radix(s, 10).unwrap()),
                preceded(
                    tag(":"),
                    map(get_2_digits, |s| u32::from_str_radix(s, 10).unwrap()),
                ),
            ))
                .parse(input)
        }

        fn get_named_tz(input: &str) -> IResult<&str, &str> {
            recognize((
                take_while1(|c: char| c.is_ascii_uppercase()),
                take_while(|c: char| {
                    is_alphanumeric(c as u8) || c == '/' || c == '-' || c == '_' || c == '+'
                }),
            ))
            .parse(input)
        }

        fn timezone(
            dt_cell: &mut ParseHint,
        ) -> impl FnMut(&str) -> IResult<&str, (FixedOffset, Tz)> {
            move |input: &str| {
                let (input, (timezone_offset, timezone_id)) = alt((
                    (recognize(get_offset), preceded(tag(" "), get_named_tz)),
                    (tag("Z"), preceded(tag(" "), get_named_tz)),
                    (tag("Z"), success("")),
                ))
                .parse(input)?;

                let timezone: (FixedOffset, chrono_tz::Tz) = match timezone_offset {
                    "Z" => (FixedOffset::east_opt(0).unwrap(), Tz::UTC),
                    _ => {
                        let (_, (sign, hours, minutes)) = get_offset(timezone_offset)?;

                        (
                            FixedOffset::east_opt(
                                sign * (hours as i32 * 3600 + minutes as i32 * 60),
                            )
                            .unwrap(),
                            get_timezone(timezone_id, dt_cell).or(Err(nom::Err::Error(Error {
                                input: input,
                                code: ErrorKind::Tag,
                            })))?,
                        )
                    }
                };

                Ok((input, timezone))
            }
        }

        pub fn datetime(dt_cell: &mut ParseHint) -> impl FnMut(&str) -> IResult<&str, HDateTime> {
            use crate::h_datetime::IntoTimezone;

            |input: &str| {
                let start = input;
                let (input, (yr, mo, day, hr, min, sec, nano, tz)) = (
                    terminated(map(take(4usize), |s| i32::from_str_radix(s, 10)), tag("-")),
                    terminated(map(take(2usize), |s| u32::from_str_radix(s, 10)), tag("-")),
                    terminated(map(take(2usize), |s| u32::from_str_radix(s, 10)), tag("T")),
                    terminated(map(take(2usize), |s| u32::from_str_radix(s, 10)), tag(":")),
                    terminated(map(take(2usize), |s| u32::from_str_radix(s, 10)), tag(":")),
                    map(take(2usize), |s| u32::from_str_radix(s, 10)),
                    opt(preceded(
                        tag("."),
                        map(digit1, |s| u32::from_str_radix(s, 10)),
                    )),
                    timezone(dt_cell),
                )
                    .parse(start)?;

                let yr = yr.or(Err(nom::Err::Error(Error {
                    input: start,
                    code: ErrorKind::Digit,
                })))?;
                let mo = mo.or(Err(nom::Err::Error(Error {
                    input: start,
                    code: ErrorKind::Digit,
                })))?;
                let day = day.or(Err(nom::Err::Error(Error {
                    input: start,
                    code: ErrorKind::Digit,
                })))?;
                let hr = hr.or(Err(nom::Err::Error(Error {
                    input: start,
                    code: ErrorKind::Digit,
                })))?;
                let min = min.or(Err(nom::Err::Error(Error {
                    input: start,
                    code: ErrorKind::Digit,
                })))?;
                let sec = sec.or(Err(nom::Err::Error(Error {
                    input: start,
                    code: ErrorKind::Digit,
                })))?;

                Ok((
                    input,
                    HDateTime::new(
                        yr,
                        mo,
                        day,
                        hr,
                        min,
                        sec,
                        if let Some(nano) = nano {
                            nano.or(Err(nom::Err::Error(Error {
                                input: start,
                                code: ErrorKind::Digit,
                            })))?
                        } else {
                            0
                        },
                        tz.into_timezone(),
                    )
                    .or(Err(nom::Err::Error(Error {
                        input: start,
                        code: ErrorKind::Digit,
                    })))?,
                ))
            }
        }

        fn coord_deg<'out, T: NumTrait + 'out>(input: &str) -> IResult<&str, T> {
            map_res(
                recognize((opt(tag("-")), digit1, opt((tag("."), digit1)))),
                |s: &str| s.parse::<T>(),
            )
            .parse(input)
        }

        pub fn coord<'out, T: NumTrait + 'out>(input: &str) -> IResult<&str, HCoord<T>> {
            let (input, (lat, long)) = (
                delimited(tag("C("), coord_deg, tag(",")),
                terminated(coord_deg, tag(")")),
            )
                .parse(input)?;

            Ok((input, HCoord::new(lat, long)))
        }

        fn tags<'out, T: NumTrait + 'out>(
            dt_cell: &mut ParseHint,
        ) -> impl FnMut(&str) -> IResult<&str, HashMap<String, HBox<'out, T>>> {
            |input: &str| {
                let (input, res) = separated_list1(
                    tag(" "),
                    (id, opt(preceded(tag(":"), literal::<T>(dt_cell)))),
                )
                .parse(input)?;

                let mut map: HashMap<String, HBox<'out, T>> = HashMap::new();

                res.into_iter().for_each(|(k, v)| {
                    map.insert(k.to_owned(), v.unwrap_or(Rc::new(HMarker) as HBox<'out, T>));
                });

                Ok((input, map))
            }
        }

        fn tags_list<'out, T: NumTrait + 'out>(
            input: &str,
        ) -> IResult<&str, Option<Vec<HBox<'out, T>>>> {
            let mut parse_hint = ParseHint::default();
            let (input, res) = opt(separated_list1(
                (
                    take_while(AsChar::is_space),
                    tag(","),
                    take_while(AsChar::is_space),
                ),
                literal::<T>(&mut parse_hint),
            ))
            .parse(input)?;

            Ok((input, res))
        }

        pub fn dict<'out, T: NumTrait + 'out>(input: &str) -> IResult<&str, HDict<'out, T>> {
            let mut parse_hint = ParseHint::default();
            let (input, opt_dict) =
                delimited(tag("{"), opt(tags::<T>(&mut parse_hint)), tag("}")).parse(input)?;

            let dict = match opt_dict {
                Some(dict) => dict,
                None => HashMap::new(),
            };

            Ok((input, HDict::from_map(dict)))
        }

        pub fn list<'out, T: NumTrait + 'out>(input: &str) -> IResult<&str, HList<'out, T>> {
            let (input, opt_vec) = delimited(tag("["), tags_list::<T>, tag("]")).parse(input)?;

            let vec = match opt_vec {
                Some(vec) => vec,
                None => Vec::new(),
            };

            Ok((input, HList::from_vec(vec)))
        }

        pub fn grid_meta<'out, T: NumTrait + 'out>(
            input: &str,
        ) -> IResult<&str, HashMap<String, HBox<'out, T>>> {
            let mut parse_hint = ParseHint::default();
            let (input, opt_dict) = opt(tags::<T>(&mut parse_hint)).parse(input)?;

            let dict = match opt_dict {
                Some(dict) => dict,
                None => Err(nom::Err::Error(Error {
                    input: input,
                    code: ErrorKind::Tag,
                }))?,
            };

            Ok((input, dict))
        }

        pub fn cols<'out, T: NumTrait + 'out>(
            input: &str,
        ) -> IResult<&str, Vec<(String, Option<HashMap<String, HBox<'out, T>>>)>> {
            let mut parse_hint = ParseHint::default();
            let (input, columns) = separated_list1(
                tag(","),
                (id, opt(preceded(space1, tags::<T>(&mut parse_hint)))),
            )
            .parse(input)?;
            let columns = columns.into_iter().map(|(id, meta)| (id.to_owned(), meta));
            let columns = columns.collect();
            Ok((input, columns))
        }

        pub fn grid_err<'out, T: NumTrait + 'out>(input: &str) -> IResult<&str, HGrid<T>> {
            let (input, _) = tag("ver:\"3.0\"")(input)?;
            let (input, meta) = delimited(space1, grid_meta::<T>, tag("\n")).parse(input)?;
            let (_, is_empty) = all_consuming(map(
                terminated(tag("empty"), take_while1(|c| c == '\n')),
                |_| true,
            ))
            .parse(input)?;
            if is_empty || meta.contains_key("err") {
                //let dis = meta.get("dis").unwrap().get_string_val().unwrap().into_string();
                let dis = match meta.get("dis") {
                    Some(v) => {
                        // v.get_string_val().unwrap().into_string()
                        if let Some(s) = v.get_string() {
                            s.clone_into_string()
                        } else {
                            return Err(nom::Err::Error(Error {
                                input: input,
                                code: ErrorKind::Tag,
                            }));
                        }
                    }
                    None => {
                        return Err(nom::Err::Error(Error {
                            input: input,
                            code: ErrorKind::Tag,
                        }));
                    }
                };

                // meta.get("errTrace").map(|s| s.get_string_val().unwrap().into_string())
                let errTrace = match meta.get("errTrace") {
                    Some(v) => {
                        // v.get_string_val().unwrap().into_string()
                        if let Some(s) = v.get_string() {
                            Some(s.clone_into_string())
                        } else {
                            return Err(nom::Err::Error(Error {
                                input: input,
                                code: ErrorKind::Tag,
                            }));
                        }
                    }
                    None => {
                        return Err(nom::Err::Error(Error {
                            input: input,
                            code: ErrorKind::Tag,
                        }));
                    }
                };

                let err = HGrid::Error { dis, errTrace };
                return Ok((input, err));
            }

            Err(nom::Err::Error(Error {
                input: input,
                code: ErrorKind::Tag,
            }))
        }

        pub fn grid<'out, T: NumTrait + 'out>(input: &str) -> IResult<&str, HGrid<'out, T>> {
            let (input, version) =
                delimited(tag("ver:\""), recognize(double), tag("\"")).parse(input)?;

            // Grid Meta
            let (input, meta) = opt(preceded(space1, grid_meta::<T>)).parse(input)?;
            let (input, _) = tag("\n").parse(input)?;
            let is_empty_res = all_consuming(terminated(
                tag::<_, _, ()>("empty"),
                take_while1(|c| c == '\n'),
            ))
            .parse(input);
            let is_empty = is_empty_res.is_ok();
            if is_empty {
                return Ok((input, HGrid::Empty { meta }));
            }

            // Cols
            let (input, columns) = terminated(cols::<T>, tag("\n")).parse(input)?;

            // Rows
            let row_width = columns.len();
            let mut parse_hint = ParseHint::default();
            let (input, rows) = separated_list1(
                tag("\n"),
                verify(
                    separated_list1(tag(","), opt(literal::<T>(&mut parse_hint))),
                    |v: &Vec<Option<HBox<T>>>| v.len() == row_width,
                ),
            )
            .parse(input)?;

            let mut grid = HGrid::from_row_vec(columns, rows);

            if let Some(meta) = meta {
                grid = grid.add_meta(meta).unwrap();
            }

            Ok((input, grid))
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::h_datetime::IntoTimezone;
            use crate::io::write::ZincWriter;
            use std::fmt::Write;

            #[test]
            fn parse_unicode_char() {
                let hello: IResult<&str, &str> = take_while(unicode_char('"'))("Hello\n\r\t\"\\");
                assert_eq!(hello, Ok(("\n\r\t\"\\", "Hello")));
            }

            #[test]
            fn parse_tags() {
                let input = "dis:\"Fri 31-Jul-2020\" view:\"chart\" title:\"Line\" chartNoScroll chartLegend:\"hide\" hisStart:2020-07-31T00:00:00-04:00 New_York hisEnd:2020-08-01T00:00:00-04:00 New_York hisLimit:10000";
                let mut parse_hint = ParseHint::default();
                let res = tags::<f64>(&mut parse_hint)(input);
                if let Ok((_, e)) = res {
                    let mut buf = String::new();

                    let v = e.get("dis").unwrap();
                    write!(buf, "{}", ZincWriter::new(v.as_ref())).unwrap();
                    let rhs = Rc::new(HStr("Fri 31-Jul-2020".to_owned())) as HBox<f64>;
                    assert_eq!(v, &rhs)
                } else {
                    panic!("Failed to parse separated list")
                }
            }

            #[test]
            fn parse_string_02() {
                assert_eq!(string("\"He\\tllo\""), Ok(("", HStr("He\tllo".to_owned()))));
            }

            #[test]
            fn parse_string_escape_dollar() {
                assert_eq!(
                    string("\"\\$equipRef \\$navName\""),
                    Ok(("", HStr("$equipRef $navName".to_owned())))
                );
            }

            #[test]
            fn parse_bool() {
                assert_eq!(boolean("T").unwrap(), ("", HBool(true)));
                assert_eq!(boolean("F").unwrap(), ("", HBool(false)),);
            }

            #[test]
            fn parse_null() {
                assert_eq!(null("N").unwrap(), ("", crate::h_null::NULL));
            }

            #[test]
            fn parse_na() {
                assert_eq!(na("NA").unwrap(), ("", crate::h_na::NA));
            }

            #[test]
            fn parse_marker() {
                assert_eq!(marker("M").unwrap(), ("", crate::h_marker::MARKER));
            }

            #[test]
            fn parse_remove() {
                assert_eq!(remove("R").unwrap(), ("", crate::h_remove::REMOVE));
            }

            #[test]
            fn parse_datetime() {
                let tz = (
                    FixedOffset::east_opt(-1 * 8 * 3600).unwrap(),
                    chrono_tz::Tz::from_str("America/Los_Angeles").unwrap(),
                );
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    datetime(&mut dt_cell)("2010-11-28T07:23:02.773-08:00 America/Los_Angeles")
                        .unwrap(),
                    (
                        "",
                        HDateTime::new(2010, 11, 28, 7, 23, 2, 773, tz.into_timezone()).unwrap()
                    )
                );
            }

            #[test]
            fn parse_datetime_02() {
                let mut dt_cell = ParseHint::default();
                let tz_offset = match FixedOffset::east_opt(-5 * 60 * 60) {
                    Some(offset) => offset,
                    None => panic!("Invalid timezone offset"),
                };
                let tz_id = match get_timezone("New_York", &mut dt_cell) {
                    Ok(tz) => tz,
                    Err(_) => panic!("Invalid timezone ID"),
                };
                let tz = (tz_offset, tz_id);
                let left =
                    HDateTime::new(2023, 3, 15, 12, 34, 56, 789, tz.into_timezone()).unwrap();
                let right = "2023-03-15T12:34:56.000000789-05:00 New_York";
                assert_eq!(datetime(&mut dt_cell)(right).unwrap(), ("", left));
            }

            #[test]
            fn parse_coord() {
                assert_eq!(coord("C(1.5,-9)").unwrap(), ("", HCoord::new(1.5, -9f64)));
            }

            #[test]
            fn parse_reference() {
                let input = "@p:demo:r:2f70054a-65a50ffa \"Carytown\"";
                let expected = HRef::new(
                    "p:demo:r:2f70054a-65a50ffa".to_owned(),
                    Some("Carytown".to_owned()),
                );
                assert_eq!(reference(input).unwrap(), ("", expected));
            }

            #[test]
            fn coerce_na2hval() {
                use crate::h_na::NA;
                let mut dt_cell = ParseHint::default();
                let (_, v) = literal::<f64>(&mut dt_cell)("NA").unwrap();
                let lhs = v.get_na();
                assert_eq!(lhs, Some(&NA))
            }

            macro_rules! assert_literal {
                ( $val: literal, $get: ident, $rhs: expr ) => {
                    let mut dt_cell = ParseHint::default();
                    let v = literal::<f64>(&mut dt_cell)($val).unwrap();
                    let lhs = v.1.$get();
                    assert_eq!(lhs, Some(&$rhs));
                };
            }

            #[test]
            fn coerce_hval() {
                use crate::h_bool::HBool;
                use crate::h_marker::MARKER;
                use crate::h_na::NA;
                use crate::h_null::NULL;
                use crate::h_remove::REMOVE;
                use crate::h_str::HStr;
                use crate::h_uri::HUri;

                assert_literal!("N", get_null, NULL);
                assert_literal!("M", get_marker, MARKER);
                assert_literal!("R", get_remove, REMOVE);
                assert_literal!("T", get_bool, HBool(true));
                assert_literal!("F", get_bool, HBool(false));
                assert_literal!("NA", get_na, NA);
                assert_literal!(
                    r#""Hello\nSmidgen\"""#,
                    get_string,
                    HStr("Hello\nSmidgen\"".to_owned())
                );
                assert_literal!(
                    "`http://www.google.com`",
                    get_uri,
                    HUri::new("http://www.google.com").unwrap()
                );
                assert_literal!(
                    "1.5kWh",
                    get_number,
                    HNumber::new(1.5f64, Some(HUnit::new("kWh".to_owned())))
                );
            }

            #[test]
            fn parse_literal_na() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("NA").unwrap().1.get_na(),
                    Some(&crate::h_na::NA)
                );
            }

            #[test]
            fn parse_literal_null() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("N").unwrap().1.get_null(),
                    Some(&crate::h_null::NULL)
                );
            }

            #[test]
            fn parse_literal_marker() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("M").unwrap().1.get_marker(),
                    Some(&crate::h_marker::MARKER)
                );
            }

            #[test]
            fn parse_literal_remove() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("R").unwrap().1.get_remove(),
                    Some(&crate::h_remove::REMOVE)
                );
            }

            #[test]
            fn parse_literal_bool() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("T").unwrap().1.get_bool(),
                    Some(&HBool(true))
                );
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("F").unwrap().1.get_bool(),
                    Some(&HBool(false))
                );
            }

            #[test]
            fn parse_literal_string() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)(r#""Hello\nWorld""#)
                        .unwrap()
                        .1
                        .get_string(),
                    Some(&HStr("Hello\nWorld".to_owned()))
                );
            }

            #[test]
            fn parse_literal_uri() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("`http://example.com`")
                        .unwrap()
                        .1
                        .get_uri(),
                    Some(&HUri::new("http://example.com").unwrap())
                );
            }

            #[test]
            fn parse_literal_number() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("42.5").unwrap().1.get_number(),
                    Some(&HNumber::new(42.5, None))
                );
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("1.5kWh")
                        .unwrap()
                        .1
                        .get_number(),
                    Some(&HNumber::new(1.5, Some(HUnit::new("kWh".to_owned()))))
                );
            }

            #[test]
            fn parse_literal_coord() {
                let mut dt_cell = ParseHint::default();
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("C(12.34,-56.78)")
                        .unwrap()
                        .1
                        .get_coord(),
                    Some(&HCoord::new(12.34, -56.78))
                );
            }

            #[test]
            fn parse_literal_datetime() {
                let mut dt_cell = ParseHint::default();
                let tz_obj = (
                    FixedOffset::east_opt(-5 * 3600).unwrap(),
                    Tz::from_str("America/New_York").unwrap(),
                );
                assert_eq!(
                    literal::<f64>(&mut dt_cell)("2023-03-15T12:34:56.789-05:00 America/New_York")
                        .unwrap()
                        .1
                        .get_datetime(),
                    Some(
                        &HDateTime::new(2023, 3, 15, 12, 34, 56, 789, tz_obj.into_timezone())
                            .unwrap()
                    )
                );
            }

            #[test]
            fn parse_literal_dict() {
                let input = r#"{key1:"value1" key2:42 key3:T}"#;
                let result = dict::<f64>(input).unwrap().1;
                assert_eq!(
                    result.get("key1").unwrap().get_string(),
                    Some(&HStr("value1".to_owned()))
                );
                assert_eq!(
                    result.get("key2").unwrap().get_number(),
                    Some(&HNumber::new(42.0, None))
                );
                assert_eq!(result.get("key3").unwrap().get_bool(), Some(&HBool(true)));
            }

            #[test]
            fn parse_literal_list() {
                let input = r#"[42,"hello" , T]"#;
                let result = list::<f64>(input).unwrap().1;
                assert_eq!(result[0].get_number(), Some(&HNumber::new(42.0, None)));
                assert_eq!(result[1].get_string(), Some(&HStr("hello".to_owned())));
                assert_eq!(result[2].get_bool(), Some(&HBool(true)));
            }

            #[test]
            fn parse_grid_empty() {
                let input = "ver:\"3.0\"\nempty\n";
                let empty_grid = grid::<f64>(input).unwrap().1;
                if let HGrid::Empty { meta } = empty_grid {
                    assert!(meta.is_none());
                } else {
                    panic!("Expected an empty grid");
                }
                //assert_eq!(grid::<f64>(input).unwrap().1, HGrid::<f64>::Empty);
            }

            #[test]
            fn parse_empty_grid_with_meta() {
                let input = "ver:\"3.0\" dis:\"Example Grid\"\nempty\n";
                let grid: HGrid<'_, f64> = grid::<f64>(input).unwrap().1;
                match grid {
                    HGrid::Empty { meta } => {
                        assert_eq!(
                            meta.unwrap().get("dis").unwrap().get_string(),
                            Some(&HStr("Example Grid".to_owned()))
                        );
                    }
                    _ => panic!("Expected an empty grid with metadata"),
                }
            }

            #[test]
            fn parse_cols() {
                let input = "col1,col2 meta1,col3 meta2";
                let result = cols::<f64>(input).unwrap().1;

                assert_eq!(result.len(), 3);

                assert_eq!(result[0].0, "col1");
                assert!(result[0].1.is_none());

                assert_eq!(result[1].0, "col2");
                assert!(result[1].1.is_some());
                assert!(result[1].1.as_ref().unwrap().contains_key("meta1"));

                assert_eq!(result[2].0, "col3");
                assert!(result[2].1.is_some());
                assert!(result[2].1.as_ref().unwrap().contains_key("meta2"));
            }

            /*
            #[test]
            fn parse_grid_with_rows() {
                let input = r#"ver:"3.0" dis:"Example Grid"
                                    col1 col2
                                    42,"hello"
                                    T,F
                                    "#;
                let grid = grid::<f64>(input).unwrap().1;
                assert_eq!(grid.meta().get("dis").unwrap().get_string(), Some(&HStr("Example Grid".to_owned())));
                assert_eq!(grid.rows()[0][0].get_number(), Some(&HNumber::new(42.0, None)));
                assert_eq!(grid.rows()[0][1].get_string(), Some(&HStr("hello".to_owned())));
                assert_eq!(grid.rows()[1][0].get_bool(), Some(&HBool(true)));
                assert_eq!(grid.rows()[1][1].get_bool(), Some(&HBool(false)));
            }
            */
        }
    }

    pub fn is_digits(chr: char) -> bool {
        is_digit(chr as u8) && chr == '_'
    }

    pub fn digits(input: &str) -> IResult<&str, (&str, &str)> {
        use nom::branch::permutation;

        permutation((digit1, take_while(is_digits))).parse(input)
    }

    pub fn exp(input: &str) -> IResult<&str, (bool, (&str, &str))> {
        use nom::sequence::pair;

        preceded(
            alt((tag("e"), tag("E"))),
            pair(
                opt(alt((tag("+"), tag("-"))))
                    .map(|sign| if let Some(c) = sign { c != "-" } else { true }),
                digits,
            ),
        )
        .parse(input)
    }

    pub fn is_unit(c: char) -> bool {
        c.is_ascii_alphabetic()
            || c >= (128 as char)
            || match c {
                '%' | '_' | '/' | '$' => true,
                _ => false,
            }
    }

    pub fn unit(input: &str) -> IResult<&str, HUnit> {
        let (input, unit_str) = take_while1(is_unit)(input)?;
        Ok((input, HUnit::new(unit_str.to_owned())))
    }

    pub fn number<'out, T: NumTrait + 'out>(input: &str) -> IResult<&str, HNumber<T>> {
        use std::slice;

        let start = input;
        let (input, is_positive) = map(opt(tag("-")), |d| d.is_none()).parse(start)?;
        let (input, integer) = digits(input)?;
        let (input, decimals) = opt(preceded(tag("."), digits)).parse(input)?;
        let (input, exponent) = opt(exp).parse(input)?;
        let number_slice = unsafe {
            slice::from_raw_parts(
                start.as_ptr(),
                (input.as_ptr() as usize) - (start.as_ptr() as usize),
            )
        };

        let number_ty: T;

        if let Ok(number_str) = std::str::from_utf8(number_slice) {
            let number_res = number_str.parse::<T>();
            if let Ok(number_ty_ok) = number_res {
                number_ty = number_ty_ok;
            } else {
                return Err(nom::Err::Error(nom::error::Error {
                    input: number_str,
                    code: nom::error::ErrorKind::Float,
                }));
            }
        } else {
            // TODO: Handle numbers with '_' in the digits
            return Err(nom::Err::Error(nom::error::Error {
                input: unsafe { std::str::from_utf8_unchecked(number_slice) },
                code: nom::error::ErrorKind::Float,
            }));
        }

        let (input, unit_opt) = opt(unit).parse(input)?;

        Ok((input, HNumber::new(number_ty, unit_opt)))
    }

    pub fn date(input: &str) -> IResult<&str, HDate> {
        let (input, year) = map(take(4usize), |s| i32::from_str_radix(s, 10)).parse(input)?;
        let year = year.or(Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Digit,
        })))?;

        let (input, _) = tag("-")(input)?;
        let (input, month) = map(take(2usize), |s| u32::from_str_radix(s, 10)).parse(input)?;
        let month = month.or(Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Digit,
        })))?;

        let (input, _) = tag("-")(input)?;
        let (input, day) = map(take(2usize), |s| u32::from_str_radix(s, 10)).parse(input)?;
        let day = day.or(Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Digit,
        })))?;

        Ok((input, HDate::new(year, month, day)))
    }

    pub fn time(input: &str) -> IResult<&str, HTime> {
        let (input, hour) = map(take(2usize), |s| u32::from_str_radix(s, 10)).parse(input)?;
        let hour = hour.or(Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Digit,
        })))?;

        let (input, _) = tag(":")(input)?;
        let (input, min) = map(take(2usize), |s| u32::from_str_radix(s, 10)).parse(input)?;
        let min = min.or(Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Digit,
        })))?;

        let (input, _) = tag(":")(input)?;
        let (input, sec) = map(take(2usize), |s| u32::from_str_radix(s, 10)).parse(input)?;
        let sec = sec.or(Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Digit,
        })))?;

        let (input, _) = tag(".")(input)?;
        let (input, nano) = map(opt(digit1), |s| {
            if let Some(s) = s {
                u32::from_str_radix(s, 10)
            } else {
                Ok(0)
            }
        })
        .parse(input)?;
        let nano = nano.or(Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Digit,
        })))?;

        Ok((input, HTime::new(hour, min, sec, nano)))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_id() {
            assert_eq!(id("asdasd1223_").unwrap(), ("", "asdasd1223_"));
        }

        #[test]
        fn parse_timezone() {
            let mut dt_cell = ParseHint::default();
            let tz = get_timezone("America/New_York", &mut dt_cell);
            let cell_tz = dt_cell.tz().take();
            assert_eq!(tz.ok(), cell_tz);
            assert_eq!(tz.ok(), Some(chrono_tz::Tz::America__New_York));

            let tz = get_timezone("New_York", &mut dt_cell);
            let cell_tz = dt_cell.tz().get().unwrap();
            assert_eq!(tz.ok(), Some(cell_tz.clone()));
            assert_eq!(tz.ok(), Some(chrono_tz::Tz::America__New_York));

            let tz = get_timezone("New_York", &mut dt_cell);
            assert_eq!(tz.ok(), Some(dt_cell.tz.get().unwrap().clone()));
            assert_eq!(tz.ok(), Some(chrono_tz::Tz::America__New_York));
        }

        #[test]
        fn parse_date() {
            assert_eq!(date("2010-11-28").unwrap(), ("", HDate::new(2010, 11, 28)));
        }

        #[test]
        fn parse_time() {
            assert_eq!(
                time("07:23:02.773").unwrap(),
                ("", HTime::new(07, 23, 02, 773))
            );
        }
    }
}
