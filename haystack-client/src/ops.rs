use std::fmt;

use tokio::sync::oneshot;

#[derive(Debug)]
pub struct HaystackOp {
    op: String,
    method: String,
    body: Option<String>,
    pub resp_tx: oneshot::Sender<HaystackResponse>
}

impl HaystackOp {
    pub fn new(op: String, method:String, body:Option<String>) -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        (Self { op, method, body, resp_tx }, resp_rx)
    }

    pub fn priv_op(&self) -> String {
        self.op.clone()
    }

    pub fn priv_method(&self) -> String {
        self.method.clone()
    }

    pub fn priv_body(&self) -> Option<String> {
        self.body.clone()
    }

    pub fn about() -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: String::from("about"),
            method: String::from("GET"),
            body: None,
            resp_tx
        };

        (op, resp_rx)
    }

    pub fn ops() -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: String::from("ops"),
            method: String::from("GET"),
            body: None,
            resp_tx
        };

        (op, resp_rx)
    }

    pub fn formats() -> (Self,oneshot::Receiver<HaystackResponse>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        let op = Self {
            op: String::from("formats"),
            method: String::from("GET"),
            body: None,
            resp_tx
        };

        (op, resp_rx)
    }
}

#[derive(Debug)]
pub enum HaystackResponse {
    Raw(String)
}

impl HaystackResponse {
    fn get_raw(self) -> String {
        let HaystackResponse::Raw(body) = self;
        body
    }
}

impl fmt::Display for HaystackResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HaystackResponse::Raw(ref body) = self;
        write!(f, "<HaystackResponse\n{}\n>",body)
    }
}