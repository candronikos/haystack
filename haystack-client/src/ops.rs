use std::{fmt::{self,Write}, str::SplitWhitespace};

use anyhow::{anyhow, Context, Error, Result};

use tokio::sync::oneshot;

#[derive(Debug)]
pub enum FStr<'a> {
    Str(&'a str),
    String(String)
}

impl FStr<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            FStr::Str(s) => s,
            FStr::String(s) => s.as_str(),
        }
    }

    pub fn split(&self) -> SplitWhitespace<'_> {
        match self {
            FStr::Str(s) => s.split_whitespace(),
            FStr::String(s) => s.split_whitespace(),
        }
    }
}

impl <'a>std::clone::Clone for FStr<'a> {
    fn clone(&self) -> Self {
        match self {
            Self::Str(arg0) => Self::Str(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
        }
    }
}

impl <'a>From<String> for FStr<'a> {
    fn from(value: String) -> Self {
        FStr::String(value)
    }
}

impl <'a>From<&'a str> for FStr<'a> {
    fn from(value: &'a str) -> Self {
        FStr::Str(value)
    }
}

impl <'a>fmt::Display for FStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FStr::Str(a) => write!(f, "{}", a),
            FStr::String(a) => write!(f, "{}", a)
        }
    }
}

pub enum Haystack {
    About,
    Close,
    Defs,
    Filetypes,
    HisRead,
    HisWrite,
    InvokeAction,
    Libs,
    Nav,
    Ops,
    PointWrite,
    Read,
    WatchPoll,
    WatchSub,
    WatchUnsub
}

const RAW_EMPTY_GRID: FStr = FStr::Str("ver:\"3.0\"\nempty");

impl Haystack {
    // fn rest(&self) -> RestOp {
    //     match self {
    //         Self::About => Self::about(),
    //         Self::Ops => Self::ops(),
    //         Self::Close => Self::close(),
    //         _ => panic!("Operation not supported")
    //     }
    // }

    pub fn about() -> RestOp {
        RestOp { op:"about".into(), method:"GET".into(), body:None }
    }

    pub fn ops() -> RestOp {
        RestOp { op:"ops".into(), method:"GET".into(), body:None }
    }

    pub fn close() -> RestOp {
        RestOp { op:"close".into(), method:"POST".into(), body:Some(RAW_EMPTY_GRID) }
    }

    pub fn formats() -> RestOp {
        RestOp { op:"formats".into(), method:"GET".into(), body:None }
    }

    pub fn read(filter: FStr, limit: Option<usize>) -> Result<RestOp,Error> {
        let mut grid = String::new();
        write!(grid,"ver:\"3.0\"\nfilter,limit\n")?;
        match limit {
            Some(lim) => write!(grid,"\"{}\",{}\n",filter,lim),
            None => write!(grid,"\"{}\",\n",filter)
        }?;

        Ok(RestOp { op:"read".into(), method:"POST".into(), body:Some(grid.into()) })
    }
}

pub struct RestOp {
    op: FStr<'static>,
    method: FStr<'static>,
    body: Option<FStr<'static>>,
}

#[derive(Debug)]
pub struct HaystackOpTxRx {
    op: FStr<'static>,
    method: FStr<'static>,
    body: Option<FStr<'static>>,
    pub resp_tx: oneshot::Sender<HaystackResponse>
}

/* impl <'a>std::clone::Clone for HaystackOp<'a> {
    fn clone(&self) -> Self {
        Self { op: self.op.clone(), method: self.method.clone(), body: self.body.clone(), resp_tx: self.resp_tx }
    }
} */

impl <'a>HaystackOpTxRx {
    pub fn new(op:FStr<'static>, method:FStr<'static>, body:Option<FStr<'static>>) -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        (Self { op, method, body, resp_tx }, resp_rx)
    }

    pub fn priv_op(&'a self) -> FStr {
        self.op.clone()
    }

    pub fn priv_method(&'a self) -> FStr {
        self.method.clone()
    }

    pub fn priv_body(&'a self) -> Option<FStr> {
        match &self.body {
            Some(x) => Some(x.to_owned()),
            None => return None,
        }
    }

    pub fn about() -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: FStr::Str("about"),
            method: FStr::Str("GET"),
            body: None,
            resp_tx
        };

        (op, resp_rx)
    }

    pub fn ops() -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: FStr::Str("ops"),
            method: FStr::Str("GET"),
            body: None,
            resp_tx
        };

        (op, resp_rx)
    }

    pub fn close() -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: FStr::Str("close"),
            method: FStr::Str("POST"),
            body: Some(FStr::Str("ver:\"3.0\"\nempty")),
            resp_tx
        };

        (op, resp_rx)
    }

    pub fn filetypes() -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: FStr::Str("formats"),
            method: FStr::Str("GET"),
            body: None,
            resp_tx
        };

        (op, resp_rx)
    }

    pub fn read(filter: FStr, limit: Option<usize>) -> Result<(Self,oneshot::Receiver<HaystackResponse>),Error> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let RestOp { op, method, body} = Haystack::read(filter, limit)?;

        let op = Self {
            op, method, body, resp_tx
        };

        Ok((op, resp_rx))
    }

    // TODO: Implment with real [HRefs]
    pub fn read_by_ids<'b, I>(ids: I) -> Result<(Self,oneshot::Receiver<HaystackResponse>)>
    where
        I: IntoIterator<Item = &'b str>,
    {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid: String = String::new();
        write!(grid,"ver:\"3.0\"\nid\n")
            .or(Err(anyhow!("Failed to write OP body")))?;

        for id in ids {
            write!(grid,"{}\n",id)
                .or(Err(anyhow!("Failed to write OP body")))?;
        }
        
        println!("grid: {}", grid);
        let op = Self {
            op: FStr::Str("read"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx
        };

        Ok((op, resp_rx))
    }

    pub fn nav(nav: Option<String>) -> Result<(Self,oneshot::Receiver<HaystackResponse>),&'a str> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid,"ver:\"3.0\"\nnavId\n")
            .or(Err("Failed to write OP body"))?;

        match nav {
            Some(n) => write!(grid,"{}\n",n),
            None => Ok(())
        }.or(Err("Failed to write OP body"))?;

        let op = Self {
            op: FStr::Str("nav"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx
        };

        Ok((op, resp_rx))
    }

    pub fn watch_sub(dis: Option<String>, id: Option<String>, lease: Option<String>, ids: Option<Vec<String>>) -> Result<(Self,oneshot::Receiver<HaystackResponse>),&'a str> {
        let (resp_tx, resp_rx) = oneshot::channel();

        if dis.is_none() && id.is_none() {
            Err("If ID is omitted, a display name must be provided")?;
        }

        let mut grid = String::new();

        write!(grid,"ver:\"3.0\"").or(Err("Failed to write OP body"))?;

        if let Some(s) = id {
            write!(grid," watchId:\"{}\"",s).or(Err("Failed to write watchId"))?;
        }

        if let Some(s) = dis {
            write!(grid," watchDis:\"{}\"",s).or(Err("Failed to write watchDis"))?;
        }

        if let Some(s) = lease {
            write!(grid," lease:{}",s).or(Err("Failed to write watch lease"))?;
        }

        if let Some(s) = ids {
            write!(grid,"\nid\n").or(Err("Failed to write OP body"))?;
            let s = s.clone().into_iter();
            write!(grid,"\n{}",s.collect::<Vec<String>>().join("\n"))
                .or(Err("Failed to write watch ids"))?;
        }

        let op = Self {
            op: FStr::Str("watchSub"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx
        };

        Ok((op, resp_rx))
    }

    pub fn his_read(id: String, date_range: String) -> Result<(Self,oneshot::Receiver<HaystackResponse>),&'a str> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid,"ver:\"3.0\"\nid,range\n{},{}",id,date_range)
            .or(Err("Failed to write OP body"))?;

        let op = Self {
            op: FStr::Str("hisRead"),    
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx
        };

        Ok((op, resp_rx))
    }

    pub fn his_write(his: String) -> Result<(Self,oneshot::Receiver<HaystackResponse>),&'a str> {
        // TODO: Implement test
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid,"{}",his)
            .or(Err("Failed to write OP body"))?;

        let op = Self {
            op: FStr::Str("hisWrite"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx
        };

        Ok((op, resp_rx))
    }
}

#[derive(Debug)]
pub enum HaystackResponse {
    Raw(String)
}

impl <'a>HaystackResponse {
    pub fn get_raw(self) -> FStr<'a> {
        let HaystackResponse::Raw(body) = self;
        FStr::String(body)
    }
}

impl <'a>fmt::Display for HaystackResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HaystackResponse::Raw(body) = self;
        write!(f, "<HaystackResponse\n{}\n>",body)
    }
}