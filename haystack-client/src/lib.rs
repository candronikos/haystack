use nom::{
    IResult,
    sequence::separated_pair,
    bytes::complete::tag,
    multi::{separated_list1,many_till},
    sequence::{pair,terminated},
    branch::alt,
    combinator::{recognize,map,peek},
    character::complete::{space0,space1,alphanumeric1,anychar},
    error::{ParseError,ErrorKind},
    InputLength
};

use scram;
use url;
use base64;
use std::fmt;
use std::str;

use std::collections::HashMap;
use std::str::FromStr;
use std::cell::RefCell;
use std::cell::Cell;

enum GridFormat {
    Zinc,
    Json,
}

#[derive(Debug)]
pub enum Error<'a> {
    RQW(reqwest::Error),
    URI(url::ParseError),
    FMT(fmt::Error),
    MSG(&'a str),
    HTTP(nom::Err<(String,nom::error::ErrorKind)>),
    SCRAM(&'a str),
    SCRAMState(scram::Error),
    SCRAMDecode(base64::DecodeError),
    SCRAMBytesToStr(std::str::Utf8Error),
}

pub struct HSession {
    uri: url::Url,
    grid_format: GridFormat,
    username: String,
    password: String,
    auth_info: RefCell<Option<HashMap<String,String>>>,
    // project: Option<String>,
    _authenticated: Cell<bool>,
    _http_client: RefCell<Option<reqwest::blocking::Client>>,
}

#[derive(Debug)]
pub enum Grid {
    Raw(String)
}

impl <'a>HSession {
    fn new(uri: &str, username: &str, password: &str/*, project: Option<String>*/) -> Result<Self,Error<'a>> {
        Ok(Self {
            uri: url::Url::parse(uri).map_err( |e| Error::URI(e))?,
            grid_format: GridFormat::Zinc,
            username: username.to_owned(),
            password: password.to_owned(),
            auth_info: RefCell::new(None),
            _authenticated: Cell::new(false),
            _http_client: RefCell::new(None),
        })
    }

    fn about(&self) -> Result<Grid,Error> {
        self._request("GET","about",None)
    }

    fn ops(&self) -> Result<Grid,Error> {
        self._request("GET","ops",None)
    }

    fn formats(&self) -> Result<Grid,Error> {
        self._request("GET","formats",None)
    }

    fn _request(&self, method:&str, op:&str, body:Option<&str>) -> Result<Grid,Error> {
        if !self._authenticated.get() {
            let () = self._blocking_authenticate()?;
        }

        let mut bearer_string = String::new();
        // let taken_auth_info = self.auth_info.borrow();
        fmt::write(&mut bearer_string,format_args!(
            "BEARER authToken={}",
            // (*taken_auth_info).as_ref()
            (*self.auth_info.borrow()).as_ref()
                .ok_or(Error::MSG("No \"authInfo\" available. This should never happen"))?
                .get("authToken").ok_or(Error::MSG("\"authToken\" missing from server response"))?
        )).map_err( |e| Error::FMT(e))?;

        let req = (*self._http_client.borrow()).as_ref()
            .ok_or(Error::MSG("Attempting request without initialising HTTP client"))?
            .request(reqwest::Method::from_str(method).map_err( |_| Error::MSG("Invalid method"))?,self.uri.clone().join(op).map_err( |e| Error::URI(e))?)
            .header("Authorization", bearer_string.as_str());

        let req = match method {
            "PUT" | "POST" | "PATCH" => req.body(
                reqwest::blocking::Body::from(
                body.ok_or(Error::MSG("Request body not provided for POST, PUT or PATCH request"))?
                .to_owned()
                )
            ),
            _ => req
        };

        let resp = req.send().map_err( |e| Error::RQW(e) )?;

        Ok(Grid::Raw(resp.text().map_err( |e| Error::RQW(e) )?))
    }

    fn _blocking_authenticate(&self) -> Result<(),Error<'a>> {
        let client = reqwest::blocking::Client::new();

        let mut uname_b64 = String::new();
        fmt::write(&mut uname_b64,format_args!(
            "HELLO username={}",
            base64::encode_config(self.username.as_bytes(),base64::STANDARD_NO_PAD))
        ).map_err( |e| Error::FMT(e))?;

        let res = client.get(self.uri.clone().join("about").map_err( |e| Error::URI(e))?)
            .header("Authorization", uname_b64.as_str())
            .send().map_err( |e| Error::RQW(e) )?;

        let www_auth_header = res.headers().get("www-authenticate")
            .ok_or(Error::MSG("Server response missing \"www-authenticate\""))?;

        let input = www_auth_header.to_str().unwrap();

        let (input,_): (&str, &str) = terminated(
            alt((tag("SCRAM"),tag("scram"))),space1
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| Error::HTTP(e.map_input(|a| a.to_owned())) )?;

        let (input,www_auth_list) = separated_list1(
            pair(tag(","),space0),
            separated_pair(alphanumeric1,tag("="),recognize(many_till(anychar,peek(alt((tag(","),eof))))))
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| Error::HTTP(e.map_input(|a| a.to_owned())) )?;

        let www_auth_map: HashMap<_, _> = www_auth_list.into_iter().collect();

        let state = scram::ScramClient::new(
            self.username.as_str(),
            self.password.as_str(),
            None
        );

        let (state, state_first) = state.client_first();

        if !www_auth_map.contains_key("hash") {
            return Err(Error::SCRAM("SHA-256 not supported"));
        } else if let Some(hash) = www_auth_map.get("hash") {
            if *hash != "SHA-256" {
                return Err(Error::SCRAM("SHA-256 not supported"));
            }
        }

        let mut data = String::new();
        fmt::write(&mut data,format_args!(
            "SCRAM handshakeToken={}, data={}",
            www_auth_map.get("handshakeToken").ok_or(Error::MSG("\"handshakeToken\" missing from server response"))?,
            base64::encode_config(state_first.as_bytes(),base64::STANDARD_NO_PAD))
        ).map_err( |e| Error::FMT(e))?;

        let res = client.get(self.uri.clone().join("about").map_err( |e| Error::URI(e))?)
            .header("Authorization", data.as_str())
            .send().map_err( |e| Error::RQW(e) )?;

        let www_auth_header = res.headers().get("www-authenticate")
            .ok_or(Error::MSG("Server response missing \"www-authenticate\""))?;

        let input = www_auth_header.to_str().unwrap();

        let (input,_): (&str, &str) = terminated(
            alt((tag("SCRAM"),tag("scram"))),space1
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| Error::HTTP(e.map_input(|a| a.to_owned())) )?;

        let (input,www_auth_list) = separated_list1(
            pair(tag(","),space0),
            separated_pair(alphanumeric1,tag("="),recognize(many_till(anychar,peek(alt((tag(","),eof))))))
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| Error::HTTP(e.map_input(|a| a.to_owned())) )?;

        let www_auth_map: HashMap<_, &str> = www_auth_list.into_iter().collect();

        let data_temp = www_auth_map.get("data").ok_or(Error::MSG("\"data\" missing from server response"))?;

        let data_temp_2 = base64::decode_config(
            str::from_utf8(data_temp.as_bytes()).map_err( |e| Error::SCRAMBytesToStr(e) )?,
            base64::STANDARD_NO_PAD
        ).map_err( |e| Error::SCRAMDecode(e) )?;

        let data_temp_1 = str::from_utf8(&data_temp_2)
            .map_err( |e| Error::SCRAMBytesToStr(e) )?;

        let state = state.handle_server_first(data_temp_1)
            .map_err( |e| Error::SCRAMState(e) )?;

        let (state, client_final) = state.client_final();

        let mut data = String::new();
        fmt::write(&mut data,format_args!(
            "SCRAM handshakeToken={}, data={}",
            www_auth_map.get("handshakeToken").ok_or(Error::MSG("\"handshakeToken\" missing from server response"))?,
            base64::encode_config(client_final.as_bytes(),base64::STANDARD_NO_PAD))
        ).map_err( |e| Error::FMT(e))?;

        let res = client.get(self.uri.clone().join("about").map_err( |e| Error::URI(e))?)
            .header("Authorization", data.as_str())
            .send().map_err( |e| Error::RQW(e) )?;

        let authentication_info = (&res.headers()).get("authentication-info")
            .ok_or(Error::MSG("Server response missing \"authentication-info\""))?;

        let input = authentication_info.to_str().unwrap();

        let (input,authentication_info_list) = separated_list1(
            pair(tag(","),space0),
            map(
                separated_pair(alphanumeric1,tag("="),recognize(many_till(anychar,peek(alt((tag(","),eof)))))),
                |(a,b):(&str,&str)| (a.to_owned(),b.to_owned())
            )
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| Error::HTTP(e.map_input(|a| a.to_owned())) )?;

        let authentication_info_map: HashMap<String, String> = authentication_info_list.into_iter().collect();

        let data_temp = authentication_info_map.get("data").ok_or(Error::MSG("\"data\" missing from server response"))?;

        let data_temp_2 = base64::decode_config(
            str::from_utf8(data_temp.as_bytes()).map_err( |e| Error::SCRAMBytesToStr(e) )?,
            base64::STANDARD_NO_PAD
        ).map_err( |e| Error::SCRAMDecode(e) )?;

        let data_temp_1 = str::from_utf8(&data_temp_2)
            .map_err( |e| Error::SCRAMBytesToStr(e) )?;

        let () = state.handle_server_final(data_temp_1)
            .map_err( |e| Error::SCRAMState(e) )?;

        *self.auth_info.borrow_mut() = Some(authentication_info_map);
        *self._http_client.borrow_mut() = Some(client);
        Ok(())
    }
}

pub fn eof<I: InputLength + Copy, E: ParseError<I>>(input: I) -> IResult<I, I, E> {
    if input.input_len() == 0 {
      Ok((input, input))
    } else {
      Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        let hs_session = HSession::new("http://localhost:8080/api/demo/","user","user",/*Some("demo".to_string())*/).unwrap();
        let auth_res = hs_session._blocking_authenticate();
        println!("{:?}",auth_res);
    }

    #[test]
    fn about()
    {
        let hs_session = HSession::new("http://localhost:8080/api/demo/","user","user",/*Some("demo".to_string())*/).unwrap();
        let Grid::Raw(res) = hs_session.about().unwrap();
        println!("{}",res);
    }

    #[test]
    fn ops()
    {
        let hs_session = HSession::new("http://localhost:8080/api/demo/","user","user",/*Some("demo".to_string())*/).unwrap();
        let Grid::Raw(res) = hs_session.ops().unwrap();
        println!("{}",res);
    }

    #[test]
    fn formats()
    {
        let hs_session = HSession::new("http://localhost:8080/api/demo/","user","user",/*Some("demo".to_string())*/).unwrap();
        let Grid::Raw(res) = hs_session.formats().unwrap();
        println!("{}",res);
    }
}
