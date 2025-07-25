use std::{
    fmt::{self, Debug, Write},
    str::SplitWhitespace,
};

use anyhow::{Error, Result, anyhow};

use tokio::sync::oneshot;

use haystack_types::{self as hs_types, NumTrait};

#[derive(Debug)]
pub enum FStr<'a> {
    Str(&'a str),
    String(String),
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

impl<'a> std::clone::Clone for FStr<'a> {
    fn clone(&self) -> Self {
        match self {
            Self::Str(arg0) => Self::Str(arg0),
            Self::String(arg0) => Self::String(arg0.clone()),
        }
    }
}

impl<'a> From<String> for FStr<'a> {
    fn from(value: String) -> Self {
        FStr::String(value)
    }
}

impl<'a> From<&'a str> for FStr<'a> {
    fn from(value: &'a str) -> Self {
        FStr::Str(value)
    }
}

impl<'a> fmt::Display for FStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FStr::Str(a) => write!(f, "{}", a),
            FStr::String(a) => write!(f, "{}", a),
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
    WatchUnsub,
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
        RestOp {
            op: "about".into(),
            method: "GET".into(),
            body: None,
        }
    }

    pub fn ops() -> RestOp {
        RestOp {
            op: "ops".into(),
            method: "GET".into(),
            body: None,
        }
    }

    pub fn close() -> RestOp {
        RestOp {
            op: "close".into(),
            method: "POST".into(),
            body: Some(RAW_EMPTY_GRID),
        }
    }

    pub fn formats() -> RestOp {
        RestOp {
            op: "formats".into(),
            method: "GET".into(),
            body: None,
        }
    }

    pub fn read(filter: FStr, limit: Option<usize>) -> Result<RestOp, Error> {
        let mut grid = String::new();
        write!(grid, "ver:\"3.0\"\nfilter,limit\n")?;
        match limit {
            Some(lim) => write!(grid, "\"{}\",{}\n", filter, lim),
            None => write!(grid, "\"{}\",\n", filter),
        }?;

        Ok(RestOp {
            op: "read".into(),
            method: "POST".into(),
            body: Some(grid.into()),
        })
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
    pub resp_tx: oneshot::Sender<HaystackResponse>,
}

/* impl <'a>std::clone::Clone for HaystackOp<'a> {
    fn clone(&self) -> Self {
        Self { op: self.op.clone(), method: self.method.clone(), body: self.body.clone(), resp_tx: self.resp_tx }
    }
} */

impl<'a> HaystackOpTxRx {
    pub fn new(
        op: FStr<'static>,
        method: FStr<'static>,
        body: Option<FStr<'static>>,
    ) -> (Self, oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        (
            Self {
                op,
                method,
                body,
                resp_tx,
            },
            resp_rx,
        )
    }

    pub fn priv_op(&'a self) -> FStr<'a> {
        self.op.clone()
    }

    pub fn priv_method(&'a self) -> FStr<'a> {
        self.method.clone()
    }

    pub fn priv_body(&'a self) -> Option<FStr<'a>> {
        match &self.body {
            Some(x) => Some(x.to_owned()),
            None => return None,
        }
    }

    pub fn about() -> (Self, oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: FStr::Str("about"),
            method: FStr::Str("GET"),
            body: None,
            resp_tx,
        };

        (op, resp_rx)
    }

    pub fn ops(
        filter: Option<FStr>,
        limit: Option<usize>,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), Error> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut payload = "ver:\"3.0\"\n".to_string();

        match filter {
            Some(f) => {
                write!(payload, "filter")?;
                if limit.is_some() {
                    write!(payload, ",limit")?;
                }
                write!(payload, "\n\"{}\"", f)?;
                if let Some(l) = limit {
                    write!(payload, ",{}", l)?;
                }
            }
            None => {
                if limit.is_some() {
                    write!(payload, "limit")?;
                } else {
                    write!(payload, "empty")?;
                }
                if let Some(l) = limit {
                    write!(payload, "\n{}", l)?;
                }
            }
        };

        write!(payload, "\n")?;

        let op = Self {
            op: FStr::Str("ops"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(payload)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn close() -> (Self, oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: FStr::Str("close"),
            method: FStr::Str("POST"),
            body: Some(FStr::Str("ver:\"3.0\"\nempty\n")),
            resp_tx,
        };

        (op, resp_rx)
    }

    pub fn defs(
        filter: Option<FStr>,
        limit: Option<usize>,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), Error> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut payload = "ver:\"3.0\"\n".to_string();

        match filter {
            Some(f) => {
                write!(payload, "filter")?;
                if limit.is_some() {
                    write!(payload, ",limit")?;
                }
                write!(payload, "\n\"{}\"", f)?;
                if let Some(l) = limit {
                    write!(payload, ",{}", l)?;
                }
            }
            None => {
                if limit.is_some() {
                    write!(payload, "limit")?;
                } else {
                    write!(payload, "empty")?;
                }
                if let Some(l) = limit {
                    write!(payload, "\n{}", l)?;
                }
            }
        };

        write!(payload, "\n")?;

        let op = Self {
            op: FStr::Str("defs"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(payload)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn libs(
        filter: Option<FStr>,
        limit: Option<usize>,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), Error> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut payload = "ver:\"3.0\"\n".to_string();

        match filter {
            Some(f) => {
                write!(payload, "filter")?;
                if limit.is_some() {
                    write!(payload, ",limit")?;
                }
                write!(payload, "\n\"{}\"", f)?;
                if let Some(l) = limit {
                    write!(payload, ",{}", l)?;
                }
            }
            None => {
                if limit.is_some() {
                    write!(payload, "limit")?;
                } else {
                    write!(payload, "empty")?;
                }
                if let Some(l) = limit {
                    write!(payload, "\n{}", l)?;
                }
            }
        };

        write!(payload, "\n")?;

        let op = Self {
            op: FStr::Str("libs"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(payload)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn filetypes(
        filter: Option<FStr>,
        limit: Option<usize>,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), Error> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut payload = "ver:\"3.0\"\n".to_string();

        match filter {
            Some(f) => {
                write!(payload, "filter")?;
                if limit.is_some() {
                    write!(payload, ",limit")?;
                }
                write!(payload, "\n\"{}\"", f)?;
                if let Some(l) = limit {
                    write!(payload, ",{}", l)?;
                }
            }
            None => {
                if limit.is_some() {
                    write!(payload, "limit")?;
                } else {
                    write!(payload, "empty")?;
                }
                if let Some(l) = limit {
                    write!(payload, "\n{}", l)?;
                }
            }
        };

        write!(payload, "\n")?;

        let op = Self {
            op: FStr::Str("filetypes"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(payload)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn read(
        filter: FStr,
        limit: Option<usize>,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), Error> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let RestOp { op, method, body } = Haystack::read(filter, limit)?;

        let op = Self {
            op,
            method,
            body,
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    // TODO: Implment with real [HRefs]
    pub fn read_by_ids<'b, I>(ids: I) -> Result<(Self, oneshot::Receiver<HaystackResponse>)>
    where
        I: IntoIterator<Item = &'b str>,
    {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid: String = String::new();
        write!(grid, "ver:\"3.0\"\nid\n").or(Err(anyhow!("Failed to write OP body")))?;

        for id in ids {
            write!(grid, "{}\n", id).or(Err(anyhow!("Failed to write OP body")))?;
        }

        let op = Self {
            op: FStr::Str("read"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn nav(nav: Option<&str>) -> Result<(Self, oneshot::Receiver<HaystackResponse>), &'a str> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid, "ver:\"3.0\"\nnavId\n").or(Err("Failed to write OP body"))?;

        match nav {
            Some(n) => write!(grid, "{}\n", n),
            None => Ok(()),
        }
        .or(Err("Failed to write OP body"))?;

        let op = Self {
            op: FStr::Str("nav"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn watch_sub<'b, I>(
        dis: Option<&str>,
        id: Option<&str>,
        lease: Option<&str>,
        ids: Option<I>,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), &'a str>
    where
        I: IntoIterator<Item = &'b str>,
    {
        let (resp_tx, resp_rx) = oneshot::channel();

        if dis.is_none() && id.is_none() {
            Err("If ID is omitted, a display name must be provided")?;
        }

        let mut grid = String::new();

        write!(grid, "ver:\"3.0\"").or(Err("Failed to write OP body"))?;

        if let Some(s) = id {
            write!(grid, " watchId:\"{}\"", s).or(Err("Failed to write watchId"))?;
        }

        if let Some(s) = dis {
            write!(grid, " watchDis:\"{}\"", s).or(Err("Failed to write watchDis"))?;
        }

        if let Some(s) = lease {
            write!(grid, " lease:{}", s).or(Err("Failed to write watch lease"))?;
        }

        write!(grid, "\nid\n").or(Err("Failed to write OP body"))?;

        if let Some(s) = ids {
            for id in s {
                write!(grid, "{}\n", id).or(Err("Failed to write OP body"))?;
            }
        }

        let op = Self {
            op: FStr::Str("watchSub"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn watch_unsub<'b, I>(
        watch_id: &str,
        ids: Option<I>,
        close: bool,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), &'a str>
    where
        I: IntoIterator<Item = &'b str>,
    {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid, "ver:\"3.0\" watchId:\"{}\"", watch_id).or(Err("Failed to write OP body"))?;

        if close {
            write!(grid, " close").or(Err("Failed to write watch close"))?;
        }

        write!(grid, "\nid\n").or(Err("Failed to write OP body"))?;

        if let Some(s) = ids {
            for id in s {
                write!(grid, "{}\n", id).or(Err("Failed to write OP body"))?;
            }
        }

        let op = Self {
            op: FStr::Str("watchUnsub"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn watch_poll<'b>(
        watch_id: &str,
        refresh: bool,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), &'a str> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let mut grid = String::new();

        write!(grid, "ver:\"3.0\" watchId:\"{}\"", watch_id).or(Err("Failed to write OP body"))?;

        if refresh {
            write!(grid, " refresh").or(Err("Failed to write watch refresh"))?;
        }

        write!(grid, "\nempty\n").or(Err("Failed to write OP body"))?;

        let op = Self {
            op: FStr::Str("watchPoll"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn his_read(
        id: &str,
        date_range: &str,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), &'a str> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid, "ver:\"3.0\"\nid,range\n{},\"{}\"", id, date_range)
            .or(Err("Failed to write OP body"))?;

        let op = Self {
            op: FStr::Str("hisRead"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn his_read_multi<'b, I>(
        ids: I,
        date_range: &str,
        timezone: Option<&str>,
    ) -> Result<(Self, oneshot::Receiver<HaystackResponse>), &'a str>
    where
        I: IntoIterator<Item = &'b str>,
    {
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid, "ver:\"3.0\" range:\"{}\"", date_range).or(Err("Failed to write OP meta"))?;

        if let Some(tz) = timezone {
            write!(grid, " tz:\"{}\"", tz).or(Err("Failed to write OP meta"))?;
        }

        write!(grid, "\nid\n").or(Err("Failed to write OP col names"))?;

        for id in ids {
            write!(grid, "{}\n", id).or(Err("Failed to write OP body"))?;
        }

        let op = Self {
            op: FStr::Str("hisRead"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }

    pub fn his_write(his: &str) -> Result<(Self, oneshot::Receiver<HaystackResponse>), &'a str> {
        // TODO: Implement test
        let (resp_tx, resp_rx) = oneshot::channel();

        let mut grid = String::new();
        write!(grid, "{}", his).or(Err("Failed to write OP body"))?;

        let op = Self {
            op: FStr::Str("hisWrite"),
            method: FStr::Str("POST"),
            body: Some(FStr::String(grid)),
            resp_tx,
        };

        Ok((op, resp_rx))
    }
}

#[derive(Debug)]
pub enum HaystackResponse {
    Raw(String),
}

impl HaystackResponse {
    pub fn get_raw(self) -> String {
        let HaystackResponse::Raw(body) = self;
        body
    }
    pub fn as_result<T>(self) -> Result<HaystackResponse>
    where
        T: NumTrait + Debug,
    {
        match self {
            HaystackResponse::Raw(ref body) => {
                match hs_types::io::parse::zinc::grid_err::<T>(body.as_str()) {
                    Ok((_input, _grid_err)) => Err(anyhow!("{}", body)),
                    Err(_e) => Ok(HaystackResponse::Raw(body.to_owned())),
                }
            }
        }
    }
}

impl<'a> fmt::Display for HaystackResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HaystackResponse::Raw(body) = self;
        write!(f, "<HaystackResponse\n{}\n>", body)
    }
}
