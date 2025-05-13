use nom::{
    branch::alt, bytes::complete::tag, character::complete::{alphanumeric1, anychar, space0, space1}, combinator::{map, peek, recognize}, error::{ErrorKind, ParseError}, multi::{many_till, separated_list1}, sequence::{pair, separated_pair, terminated}, Err, IResult, InputLength
};

use futures::{future::{AbortHandle, Abortable}, TryFutureExt};

use scram;
use url;
use base64;
use std::fmt;
use std::str;

use std::collections::HashMap;
use std::str::FromStr;

use std::sync::{Arc};
use tokio::sync::{Mutex,Semaphore};

use tokio::sync::mpsc;
use anyhow::{anyhow, Result as AnyResult};

pub mod ops;
use ops::{FStr, HaystackOpTxRx, HaystackResponse};

static BASE64_CONFIG: base64::Config = base64::URL_SAFE_NO_PAD;

#[derive(Clone,Copy)]
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

pub struct HSession {
    uri: url::Url,
    grid_format: GridFormat,
    username: String,
    password: String,
    auth_info: Arc<Mutex<Option<String>>>,
    // semaphore: Arc<Semaphore>,
    _http_client: reqwest::Client,
}

#[derive(Debug)]
pub enum Grid {
    Raw(String)
}

async fn new_hs_session<'a>(uri: String, username: String, password: String, accept_invalid_certs: bool, existing_session: Arc<Mutex<Option<String>>>, buffer: Option<usize>) -> Result<(AbortHandle,mpsc::Sender<HaystackOpTxRx>, Option<String>),Error<'a>> {
    let (tx, mut rx) = mpsc::channel::<HaystackOpTxRx>(buffer.unwrap_or(10000));

    let uri_member = url::Url::parse(&uri).map_err( |e| Error::URI(e))?;
    let mut http_client = reqwest::Client::builder();
    
    http_client = http_client
        .danger_accept_invalid_certs(accept_invalid_certs);
    
    let http_client = http_client
        .build()
        .unwrap();
    
    let mut hs_session = HSession {
        uri: uri_member,
        grid_format: GridFormat::Zinc,
        username: username,
        password: password,
        auth_info: existing_session,
        // semaphore: Arc::new(Semaphore::new(1)),
        _http_client: http_client,
    };

    let auth_token: Option<String>;
    if !hs_session.is_authenticated().await {
        auth_token = Some(hs_session._authenticate().await.unwrap());
    } else {
        auth_token = None;
    }

    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(async move {

        let semaphore = Arc::new(Semaphore::new(1));
        while let Some(op) = rx.recv().await {
            let _permit = semaphore.clone().acquire_owned().await.unwrap();
            if !hs_session.is_authenticated().await {
                let _ = hs_session._authenticate().await.unwrap();
                // drop(_permit);
            }
            drop(_permit);

            let ctx = HTTPContext {
                client: hs_session._http_client.clone(),
                auth_info: hs_session.auth_info.clone(),
                uri: hs_session.uri.clone(),
                grid_format: hs_session.grid_format.clone(),
            };

            tokio::spawn(async move {
                match HSession::_request(ctx,op).await {
                    Ok((op,res)) => {
                        if op.resp_tx.is_closed() {
                            //panic!("Sender for Response CLOSED. Won't be able to send");
                            //return Err(anyhow!("Sender for Response CLOSED. Won't be able to send"));
                            eprintln!("Sender for Response CLOSED. Won't be able to send");
                        }
                        // let sent_resp_res = op.resp_tx.send(HaystackResponse::Raw(res.to_string()));
                        if let Err(e) = op.resp_tx.send(HaystackResponse::Raw(res.to_string())) {
                            //panic!("Handling failed requests to channel not supported!");
                            //return Err(anyhow!("Handling failed requests to channel not supported! {}",e));
                            eprintln!("Handling failed requests to channel not supported! {}",e);
                        };
                        //Ok(())
                    },
                    Err(e) => {
                        //panic!("Handling failed requests not supported!");
                        //Err(anyhow!("Handling failed requests not supported! {:?}",e))
                        eprintln!("Handling failed requests not supported! {:?}",e);
                    }
                }
            });
        }
    }, abort_registration);

    tokio::spawn(future);

    Ok((abort_handle,tx, auth_token))
}

struct HTTPContext {
    client: reqwest::Client,
    auth_info: Arc<Mutex<Option<String>>>,
    uri: url::Url,
    grid_format: GridFormat,
}

impl <'a>HSession {
    pub async fn new(uri: String, username: String, password: String, accept_invalid_certs: bool, existing_session: Arc<tokio::sync::Mutex<Option<String>>>, buffer: Option<usize>) -> Result<(AbortHandle,mpsc::Sender<HaystackOpTxRx>, Option<String>),Error<'a>> {
        let mut url = url::Url::parse(&uri).map_err( |e| Error::URI(e))?;
        if !url.path().ends_with('/') {
            url.path_segments_mut()
                .expect("Cannot modify URL path segments")
                .push("");
        }
        new_hs_session(url.to_string(), username, password, accept_invalid_certs, existing_session, buffer).await
    }

    async fn _request(ctx: HTTPContext, haystack_op: HaystackOpTxRx) -> AnyResult<(HaystackOpTxRx,FStr<'a>)> {
        let (method, op, body) = (haystack_op.priv_method(),haystack_op.priv_op(),haystack_op.priv_body());
        //let auth_clone = ctx.auth_info.clone().lock_owned().await;
        //let auth: String = auth_clone.to_owned().expect("No auth method in haystack session object");
        let auth = {
            let auth_clone = ctx.auth_info.clone().lock_owned().await;
            auth_clone.to_owned().ok_or_else(|| anyhow!("No auth method in haystack session object"))?
        };
        
        let req = ctx.client
            .request(reqwest::Method::from_str(method.as_str())
                            .map_err( |_| anyhow!("MSG: Invalid method"))?,ctx.uri.clone().join(op.as_str())
                            .map_err( |e| anyhow!("Error::URI: {:?}",e))?)
            .header("Authorization", auth);

        let req = match method.as_str() {
            "PUT" | "POST" | "PATCH" => req.body(
                reqwest::Body::from(
                body.ok_or(anyhow!("Request body not provided for POST, PUT or PATCH request"))?.to_string()
                )
            ),
            _ => req
        };

        let req = match ctx.grid_format {
            GridFormat::Zinc => req.header("Content-Type","text/zinc"),
            GridFormat::Json => req.header("Content-Type","application/json"),
        };

        let resp = req.send().await.map_err( |e| anyhow!("Error::RQW({})",e))?;

        let res = resp.text().await.map_err( |e| anyhow!("Error::RQW({})",e))?;
        Ok((haystack_op,res.into()))
    }

    async fn _authenticate(&mut self) -> AnyResult<String> {
        // let client = reqwest::Client::new();
        let client = self._http_client.clone();

        let mut uname_b64 = String::new();
        fmt::write(&mut uname_b64,format_args!(
            "HELLO username={}",
            base64::encode_config(self.username.as_bytes(),BASE64_CONFIG))
        )
        .or_else(|e| Err(anyhow!("Auth Error: Unable to format HELLO msg: \"{:?}\"",e)))?;

        //let res = client.get(self.uri.clone().join("about").map_err( |e| Error::URI(e))?)
        let url = self.uri.clone().join("about").map_err( |e| anyhow!("{:?}",e))?;
        let res = client.get(url)
            .header("Authorization", uname_b64.as_str())
            .send().await.or_else(|e| Err(anyhow!("Auth Error: {:?}",e)))?;

        let www_auth_header = res.headers().get("www-authenticate")
            .ok_or(anyhow!("Server response missing \"www-authenticate\" 1:\nHEADERS: {:?}", res.headers()))?;

        let input = www_auth_header.to_str()
            .map_err(|e| anyhow!("http::header::value::ToStrError: {:?}", e))?;

        let (input,_): (&str, &str) = terminated(
            alt((tag("SCRAM"),tag("scram"))),space1
        )(input).map_err(|e: nom::Err<(&str,nom::error::ErrorKind)>| anyhow!("{:?}",e))?;

        let (input,www_auth_list) = separated_list1(
            pair(tag(","),space0),
            separated_pair(alphanumeric1,tag("="),recognize(many_till(anychar,peek(alt((tag(","),eof))))))
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| anyhow!("{:?}",e))?;

        let www_auth_map: HashMap<_, _> = www_auth_list.into_iter().collect();

        let state = scram::ScramClient::new(
            self.username.as_str(),
            self.password.as_str(),
            None
        );

        let (state, state_first) = state.client_first();

        if !www_auth_map.contains_key("hash") {
            return Err(anyhow!("SCRAM: SHA-256 not supported"));
        } else if let Some(hash) = www_auth_map.get("hash") {
            if *hash != "SHA-256" {
                return Err(anyhow!("SCRAM: SHA-256 not supported"));
            }
        }

        let mut data = String::new();
        // TODO: Remove commented lines
        // fmt::write(&mut data,format_args!(
        //     "SCRAM handshakeToken={}, data={}",
        //     www_auth_map.get("handshakeToken").ok_or(Error::MSG("\"handshakeToken\" missing from server response"))?,
        //     base64::encode_config(state_first.as_bytes(),BASE64_CONFIG))
        // ).map_err( |e| Error::FMT(e))?;

        if let Some(hs_token) = www_auth_map.get("handshakeToken") {
            fmt::write(&mut data,format_args!(
                "SCRAM handshakeToken={}, data={}",
                hs_token,
                base64::encode_config(state_first.as_bytes(),BASE64_CONFIG))
            ).map_err( |e| anyhow!("{:?}",e))?;
        } else {
            fmt::write(&mut data,format_args!(
                "SCRAM data={}",
                base64::encode_config(state_first.as_bytes(),BASE64_CONFIG))
            ).map_err( |e| anyhow!("{:?}",e))?;
        }

        let res = client.get(self.uri.clone().join("about").map_err( |e| anyhow!("{:?}",e))?)
            .header("Authorization", data.as_str())
            .send().await.map_err( |e| anyhow!("{:?}",e))?;

        let www_auth_header = res.headers().get("www-authenticate")
            .ok_or(anyhow!("Server response missing \"www-authenticate\" 2:\nHEADERS: {:?}", res.headers()))?;

        let input = www_auth_header.to_str().unwrap();

        let (input,_): (&str, &str) = terminated(
            alt((tag("SCRAM"),tag("scram"))),space1
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| anyhow!("{:?}",e))?;

        let (input,www_auth_list) = separated_list1(
            pair(tag(","),space0),
            separated_pair(alphanumeric1,tag("="),recognize(many_till(anychar,peek(alt((tag(","),eof))))))
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| anyhow!("{:?}",e))?;

        let www_auth_map: HashMap<_, &str> = www_auth_list.into_iter().collect();

        let data_temp = www_auth_map.get("data").ok_or(anyhow!("\"data\" missing from server response"))?;

        let data_temp_2 = base64::decode_config(
            str::from_utf8(data_temp.as_bytes()).map_err( |e| anyhow!("SCRAM bytes to str: {:?}",e))?,
            BASE64_CONFIG
        ).map_err( |e| anyhow!("SCRAM decode: {:?}, payload: {:02X?}",e,data_temp.as_bytes()))?;

        let data_temp_1 = str::from_utf8(&data_temp_2)
            .map_err( |e| anyhow!("SCRAM bytes to str: {:?}",e))?;

        let state = state.handle_server_first(data_temp_1)
            .map_err( |e| anyhow!("SCRAM state: {:?}",e))?;

        let (state, client_final) = state.client_final();

        let mut data = String::new();
        // TODO: Remove commented lines
        // fmt::write(&mut data,format_args!(
        //     "SCRAM handshakeToken={}, data={}",
        //     www_auth_map.get("handshakeToken").ok_or(Error::MSG("\"handshakeToken\" missing from server response"))?,
        //     base64::encode_config(client_final.as_bytes(),BASE64_CONFIG))
        // ).map_err( |e| Error::FMT(e))?;

        if let Some(hs_token) = www_auth_map.get("handshakeToken") {
            fmt::write(&mut data,format_args!(
                "SCRAM handshakeToken={}, data={}",
                hs_token,
                base64::encode_config(client_final.as_bytes(),BASE64_CONFIG))
            ).map_err( |e| anyhow!("Format: {:?}",e))?;
        } else {
            fmt::write(&mut data,format_args!(
                "SCRAM data={}",
                base64::encode_config(client_final.as_bytes(),BASE64_CONFIG))
            ).map_err( |e| anyhow!("Format: {:?}",e))?;
        }

        let res = client.get(self.uri.clone().join("about").map_err( |e| anyhow!("URI: {:?}",e))?)
            .header("Authorization", data.as_str())
            .send().await.map_err( |e| anyhow!("RQW: {:?}",e))?;

        let authentication_info = (&res.headers()).get("authentication-info")
            .ok_or(anyhow!("Server response missing \"authentication-info\": HEADERS: {:?}\nSTATUS: {:?}", res.headers(), res.status()))?;

        let input = authentication_info.to_str().unwrap();

        let (input,authentication_info_list) = separated_list1(
            pair(tag(","),space0),
            map(
                separated_pair(alphanumeric1,tag("="),recognize(many_till(anychar,peek(alt((tag(","),eof)))))),
                |(a,b):(&str,&str)| (a.to_owned(),b.to_owned())
            )
        )(input).map_err( |e: nom::Err<(&str,nom::error::ErrorKind)>| anyhow!("HTTP: {:?}",e))?;

        let authentication_info_map: HashMap<String, String> = authentication_info_list.into_iter().collect();

        let data_temp = authentication_info_map.get("data").ok_or(anyhow!("\"data\" missing from server response"))?;

        let data_temp_2 = base64::decode_config(
            str::from_utf8(data_temp.as_bytes()).map_err( |e| anyhow!("SCRAM bytes to str: {:?}",e))?,
            BASE64_CONFIG
        ).map_err( |e| anyhow!("SCRAM decode: {:?}, payload: {:02X?}",e,data_temp.as_bytes()))?;

        let data_temp_1 = str::from_utf8(&data_temp_2)
            .map_err( |e| anyhow!("SCRAM bytes to str: {:?}",e))?;

        let () = state.handle_server_final(data_temp_1)
            .map_err( |e| anyhow!("SCRAM state: {:?}",e))?;

        let mut bearer_string = String::new();
        fmt::write(&mut bearer_string,format_args!(
            "BEARER authToken={}",
            authentication_info_map.get("authToken")
            .ok_or(anyhow!("\"authToken\" missing from server response"))?
        )).map_err( |e| anyhow!("Format: {:?}",e))?;

        *self.auth_info.clone().lock_owned().await = Some(bearer_string.clone());
        Ok(bearer_string)
    }

    pub async fn is_authenticated(&self) -> bool {
        self.auth_info.lock().await.is_some()
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
    use futures::future;
    use std::ops::{Deref, DerefMut};
    use super::*;

    // #[fixture]
    // // TODO: Write test with close op that closes original session
    // fn client() -> future::Ready<Box<mpsc::Sender<ops::HaystackOpTxRx>>> {
    //     lazy_static! {
    //         static ref HS_SESSION: (AbortHandle,mpsc::Sender<ops::HaystackOpTxRx>) = HSession::new(
    //             "https://analytics.host.docker.internal/api/demo/".to_owned(),
    //             "su".to_owned(),
    //             "password".to_owned(),
    //             true,
    //             Arc::new(Mutex::new(None)),
    //             None
    //         ).unwrap();
    //     }
    //     future::ready::<Box<mpsc::Sender<ops::HaystackOpTxRx>>>(Box::new(HS_SESSION.1.clone()))
    // }

    // TODO: Write test with close op that closes original session
    #[fixture]
    async fn client() -> Box<mpsc::Sender<ops::HaystackOpTxRx>> {
        let hs_session: (AbortHandle, mpsc::Sender<ops::HaystackOpTxRx>, Option<String>) = HSession::new(
                "https://analytics.host.docker.internal/api/demo/".to_owned(),
                "su".to_owned(),
                "password".to_owned(),
                true,
                Arc::new(Mutex::new(None)),
                None
            ).await.unwrap();
        Box::new(hs_session.1)
    }

    #[rstest]
    #[tokio::test]
    async fn about<D,F>(client: F)
        where F: std::future::Future<Output = D>,
            D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
        let (op,resp) = HaystackOpTxRx::about();

        let client_res = client.await;
        let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
        let res = permit.send(op);

        if let Err(e) = resp.await {
            panic!("Failed to receive response: {}",e);
        }
    }

    #[rstest]
    #[tokio::test]
    async fn ops<D, F>(client: F)
    	where F: std::future::Future<Output = D>,
        	D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
        let (op,resp) = HaystackOpTxRx::ops(None, None).unwrap();

        let client_res = client.await;
        let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
        let res = permit.send(op);

        if let Err(e) = resp.await {
            panic!("Failed to receive response: {}",e);
        }
    }

    #[rstest]
    #[tokio::test]
    async fn filetypes<D, F>(client: F)
   		where F: std::future::Future<Output = D>,
        	D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
        let (op,resp) = HaystackOpTxRx::filetypes(None,None).unwrap();

        let client_res = client.await;
        let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
        let res = permit.send(op);

        if let Err(e) = resp.await {
            panic!("Failed to receive response: {}",e);
        }
    }

    #[rstest]
    #[tokio::test]
    async fn read<D, F>(client: F)
    	where F: std::future::Future<Output = D>,
        	D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
        let (op,resp) = HaystackOpTxRx::read("point and his and temp".into(), Some(10)).unwrap();

        let client_res = client.await;
        let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
        let res = permit.send(op);

        if let Err(e) = resp.await {
            panic!("Failed to receive response: {}",e);
        }
    }

    #[rstest]
    #[tokio::test]
    async fn nav_root<D, F>(client: F)
    	where F: std::future::Future<Output = D>,
        	D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
        let (op,resp) = HaystackOpTxRx::nav(None).unwrap();

        let client_res = client.await;
        let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
        let res = permit.send(op);

        if let Err(e) = resp.await {
            panic!("Failed to receive response: {}",e);
        }
    }

    #[rstest]
    #[tokio::test]
    async fn nav_site<D, F>(client: F)
    	where F: std::future::Future<Output = D>,
       		D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
        let (op,resp) = HaystackOpTxRx::nav(Some("`equip:/Carytown`")).unwrap();

        let client_res = client.await;
        let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
        let res = permit.send(op);

        if let Err(e) = resp.await {
            panic!("Failed to receive response: {}",e);
        }
    }

    // #[rstest]
    // #[tokio::test]
    // async fn reuse_with_multi_op<D, F>(client: F)
    //     where F: std::future::Future<Output = D>,
    //     	D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
    //     let (op,resp) = HaystackOpTxRx::nav(Some("`equip:/Carytown`")).unwrap();
    //     let client_res = client.clone().await;
    //     let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
    //     let res = permit.send(op);

    //     let _response = resp.await.unwrap();

    //     let (op,resp) = HaystackOpTxRx::about();
    //     let client_res = client.clone().await;
    //     let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
    //     let res = permit.send(op);

    //     let response = resp.await.unwrap();
    // }

    #[rstest]
    #[tokio::test]
    async fn his_read<D, F>(client: F)
    	where F: std::future::Future<Output = D>,
        	D: DerefMut<Target = mpsc::Sender<ops::HaystackOpTxRx>> {
        let (op,resp) = HaystackOpTxRx::his_read("@p:demo:r:26464231-bea9f430","2019-01-01").unwrap();

        let client_res = client.await;
        let permit = client_res.reserve().await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
        let res = permit.send(op);

        let response = resp.await.or_else(|e| Err(anyhow!("Failed to reserve permit: {}",e))).unwrap();
    }

    // TODO: Close session
    #[tokio::test]
    async fn spawn_multiple_tasks_in_new_session() {
        use futures::join;
        let (abort_handle,addr, _) = HSession::new(
            "https://analytics.host.docker.internal/api/demo/".to_owned(),
            "su".to_owned(),
            "password".to_owned(),
            true,
            Arc::new(Mutex::new(None)),
            None
        ).await.unwrap();

        let (nav_op,nav_resp) = HaystackOpTxRx::nav(None).unwrap();
        let (formats_op,formats_resp) = HaystackOpTxRx::filetypes(None, None).unwrap();
        let (about_op,about_resp) = HaystackOpTxRx::about();

        let mut nav_addr = addr.clone();
        let mut formats_addr = addr.clone();
        let mut about_addr = addr.clone();

        let (nav_res,formats_res,about_res) = join!(
            nav_addr.send(nav_op),
            formats_addr.send(formats_op),
            about_addr.send(about_op),
        );

        if nav_res.is_err() || formats_res.is_err() || about_res.is_err() {
            panic!("One or more requests failed 1")
        }

        let (nav_res,formats_res,about_res) = join!(nav_resp, formats_resp, about_resp);

        if nav_res.is_err() || formats_res.is_err() || about_res.is_err() {
            panic!("One or more requests failed 2")
        }

        abort_handle.abort()
    }
}