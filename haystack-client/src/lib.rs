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

use futures::future::{Abortable, AbortHandle};

use scram;
use url;
use base64;
use std::fmt;
use std::str;

use std::collections::HashMap;
use std::str::FromStr;

use tokio::sync::mpsc;

pub mod ops;
use ops::{HaystackOp,HaystackResponse};

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
    HaystackErr,
    PoisonedLock(&'a str)
}

struct HSession {
    uri: url::Url,
    grid_format: GridFormat,
    username: String,
    password: String,
    auth_info: Option<String>,
    _authenticated: bool,
    _http_client: Option<reqwest::Client>,
}

#[derive(Debug)]
pub enum Grid {
    Raw(String)
}

fn new_hs_session<'a>(uri: String, username: String, password: String, buffer: Option<usize>) -> Result<(AbortHandle,mpsc::Sender<HaystackOp>),Error<'a>> {
    let (tx, mut rx) = mpsc::channel::<HaystackOp>(buffer.unwrap_or(100));

    let uri_member = url::Url::parse(&uri).map_err( |e| Error::URI(e))?;

    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(async move {
        let mut obj = HSession {
            uri: uri_member,
            grid_format: GridFormat::Zinc,
            username: username,
            password: password,
            auth_info: None,
            _authenticated: false,
            _http_client: None,
        };
        while let Some(op) = rx.recv().await {
            println!("\nCLIENT PTR: {:p}",&obj);
            println!("REQUEST:{:?}",op);
            println!("ISAUTHENTICATED: {:?}",obj._authenticated);
            if !obj._authenticated {
                let () = obj._authenticate().await.unwrap();
            }
            let result = (&mut obj)._request(op.priv_method(),op.priv_op(),op.priv_body()).await;
            println!("RESULT: {:?}\n",result);
            if let Ok(res) = result {
                let sent_resp_res = op.resp_tx.send(HaystackResponse::Raw(res));
                if let Err(e) = sent_resp_res {
                    // println!("FAILED RESPONSE: {:?}\n",e);
                    // panic!("Handling failed requests to channel not supported!");
                }
            } else if let Err(e) = result {
                // println!("FAILED ERROR: {:?}\n",e);
                // panic!("Handling failed requests not supported!");
            }
        }
    }, abort_registration);

    tokio::spawn(future);

    Ok((abort_handle,tx))
}

impl <'a>HSession {
    // fn new(uri: &str, username: &str, password: &str, buffer: Option<usize>/*, project: Option<String>*/) -> Result<Self,Error<'a>> {
    fn new(uri: String, username: String, password: String, buffer: Option<usize>) -> Result<(AbortHandle,mpsc::Sender<HaystackOp>),Error<'a>> {
        new_hs_session(uri, username, password, buffer)
    }

    async fn _request(&mut self, method:String, op:String, body:Option<String>) -> Result<String,Error<'a>> {
        let bearer_string: String;
        {
            bearer_string = self.auth_info.as_ref()
                .ok_or(Error::MSG("No \"authInfo\" available. This should never happen"))?
                .clone();
        }

        let req = self._http_client.as_ref()
            .ok_or(Error::MSG("Attempting request without initialising HTTP client"))?
            .request(reqwest::Method::from_str(method.as_str()).map_err( |_| Error::MSG("Invalid method"))?,self.uri.clone().join(op.as_str()).map_err( |e| Error::URI(e))?)
            .header("Authorization", bearer_string);

        let req = match method.as_str() {
            "PUT" | "POST" | "PATCH" => req.body(
                reqwest::Body::from(
                body.ok_or(Error::MSG("Request body not provided for POST, PUT or PATCH request"))?
                )
            ),
            _ => req
        };

        let req = match self.grid_format {
            GridFormat::Zinc => req.header("Content-Type","text/zinc"),
            GridFormat::Json => req.header("Content-Type","application/json"),
        };

        let resp = req.send().await.map_err( |e| Error::RQW(e) )?;

        Ok(resp.text().await.map_err( |e| Error::RQW(e) )?)
    }

    async fn _authenticate(&mut self) -> Result<(),Error<'a>> {
        let client = reqwest::Client::new();

        let mut uname_b64 = String::new();
        fmt::write(&mut uname_b64,format_args!(
            "HELLO username={}",
            base64::encode_config(self.username.as_bytes(),base64::STANDARD_NO_PAD))
        ).map_err( |e| Error::FMT(e))?;

        let res = client.get(self.uri.clone().join("about").map_err( |e| Error::URI(e))?)
            .header("Authorization", uname_b64.as_str())
            .send().await.map_err( |e| Error::RQW(e) )?;

        let www_auth_header = res.headers().get("www-authenticate")
            .ok_or(Error::MSG("Server response missing \"www-authenticate\""))?;

        let input = www_auth_header.to_str().map_err( |e| Error::MSG("http::header::value::ToStrError") )?;

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
            .send().await.map_err( |e| Error::RQW(e) )?;

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
            .send().await.map_err( |e| Error::RQW(e) )?;

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

        let mut bearer_string = String::new();
        fmt::write(&mut bearer_string,format_args!(
            "BEARER authToken={}",
            authentication_info_map.get("authToken")
            .ok_or(Error::MSG("\"authToken\" missing from server response"))?
        )).map_err( |e| Error::FMT(e) )?;

        self.auth_info = Some(bearer_string);
        self._http_client = Some(client);
        self._authenticated = true;
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
    use rstest::*;
    use std::sync::Arc;
    use futures::lock::Mutex;

    use super::*;

    #[fixture]
    fn client() -> Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>> {
        let (abort_handle,addr) = HSession::new(
            "http://localhost:8080/api/demo/".to_owned(),
            "user".to_owned(),
            "user".to_owned(),
            None
        ).unwrap();
        Arc::new(Mutex::new((abort_handle, addr)))
    }

    #[rstest]
    #[tokio::test]
    async fn about(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::about();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        if let Err(e) = resp.await {
            panic!("Failed to receive response");
        }
    }

    #[rstest]
    #[tokio::test]
    async fn ops(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::ops();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        if let Err(e) = resp.await {
            panic!("Failed to receive response");
        }
    }

    #[rstest]
    #[tokio::test]
    async fn formats(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::formats();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        if let Err(e) = resp.await {
            panic!("Failed to receive response");
        }
    }

    #[rstest]
    #[tokio::test]
    async fn read(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::read("point and his and temp".to_owned(), Some(10)).unwrap();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        if let Err(e) = resp.await {
            panic!("Failed to receive response");
        }
    }

    #[rstest]
    #[tokio::test]
    async fn nav_root(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::nav(None).unwrap();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        if let Err(e) = resp.await {
            panic!("Failed to receive response");
        }
    }

    #[rstest]
    #[tokio::test]
    async fn nav_site(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::nav(Some("`equip:/Carytown`".to_owned())).unwrap();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        if let Err(e) = resp.await {
            panic!("Failed to receive response");
        }
    }

    #[rstest]
    #[tokio::test]
    async fn reuse_with_multi_op(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::nav(Some("`equip:/Carytown`".to_owned())).unwrap();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        let response = resp.await.unwrap();

        let (op,resp) = HaystackOp::about();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        let response = resp.await.unwrap();
    }

    #[rstest]
    #[tokio::test]
    async fn his_read(client: Arc<Mutex<(AbortHandle,mpsc::Sender<ops::HaystackOp>)>>) {
        let (op,resp) = HaystackOp::his_read("@p:demo:r:26464231-bea9f430".to_owned(),"\"2019-01-01\"".to_owned()).unwrap();
        let res = client.lock().await.1.send(op).await;

        if let Err(e) = res {
            panic!("Failed to send request");
        }

        let response = resp.await.unwrap();
    }
}