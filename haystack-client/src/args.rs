use std::{io::{self, Read, Write}, path::PathBuf, pin::Pin};

use anyhow::{Error, Result as AnyResult};
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, Parser, Subcommand};
use futures::stream::{AbortHandle, Aborted, Any};
use haystack_types::NumTrait;
use haystackclientlib::ops::{FStr, HaystackOpTxRx, HaystackResponse};
use reedline_repl_rs::{AsyncCallback, Repl};
use saphyr::AnnotatedMapping;
use tokio::sync::{oneshot::Receiver,mpsc::Sender};
use url::{Url, Host};

use is_terminal::IsTerminal;

/*
ops().toRecList.map(r => "OP { def:\""+r->def+"\",doc:\""+r["doc"]+"\",is:\""+r->is+"\",lib:\""+r->lib+"\",no_side_effects:\""+r.has("no_side_effects")+"\",nodoc:\""+r.has("nodoc")+"\",type_name:\""+r["type_name"]+"\" }") // def,doc,is,lib,no_side_effects,nodoc,type_name
*/

type REPL_FUNC_TYPE<T> = fn(ArgMatches, &mut T) -> AnyResult<(HaystackOpTxRx, Receiver<HaystackResponse>)>;
type MATCH_FUNC_TYPE = fn(&ArgMatches) -> AnyResult<(HaystackOpTxRx, Receiver<HaystackResponse>)>;

pub struct IsTTY {
    pub stdin: bool,
    pub stdout: bool,
    pub stderr: bool,
}

impl IsTTY {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin().is_terminal(),
            stdout: io::stdout().is_terminal(),
            stderr: io::stderr().is_terminal(),
        }
    }
    pub fn all(&self) -> bool {
        self.stdin && self.stdout && self.stderr
    }
    pub fn any(&self) -> bool {
        self.stdin || self.stdout || self.stderr
    }
}

pub struct OP {
    def: &'static str,
    doc: &'static str,
    is: &'static str,
    lib: &'static str,
    no_side_effects: bool,
    nodoc: bool,
    type_name: &'static str,
    cmd: Option<&'static dyn Fn(&OP) -> Command>,
}

fn cmd_generic(op:&OP) -> Command {
    let mut cmd = Command::new(&op.def[3..]);
    if !op.nodoc {
        cmd = cmd.about(op.doc);
    }
    cmd
}

fn cmd_nav(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("nav")
            .value_name("navId")
            .index(1)
            .required(false)
            //.value_parser(value_parser!(String))
            //.num_args(0..1)
            .help("The node in the navigation tree to query"));
    cmd
}

fn cmd_defs(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("filter")
            .action(ArgAction::Set)
            .required(false)
            .help("The filter to apply to the defs operation"))
        .arg(Arg::new("limit")
            .long("limit")
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize))
            .num_args(1)
            .help("The maximum number of defs to return"));
    cmd
}

fn cmd_filetypes(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("filter")
            .action(ArgAction::Set)
            .required(false)
            .help("The filter to apply to the filetypes operation"))
        .arg(Arg::new("limit")
            .long("limit")
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize))
            .num_args(1)
            .help("The maximum number of filetypes to return"));
    cmd
}

fn cmd_ops(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("filter")
            .action(ArgAction::Set)
            .required(false)
            .help("The filter to apply to the filetypes operation"))
        .arg(Arg::new("limit")
            .long("limit")
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize))
            .num_args(1)
            .help("The maximum number of filetypes to return"));
    cmd
}

fn cmd_libs(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("filter")
            .action(ArgAction::Set)
            .required(false)
            .help("The filter to apply to the filetypes operation"))
        .arg(Arg::new("limit")
            .long("limit")
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize))
            .num_args(1)
            .help("The maximum number of filetypes to return"));
    cmd
}

fn cmd_read(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("filter")
            .long("filter")
            .action(ArgAction::Set)
            .num_args(1)
            .conflicts_with("ids")
            .help("The filter to apply to the read operation"))
        .arg(Arg::new("limit")
            .long("limit")
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize))
            .num_args(1)
            .conflicts_with("ids")
            .help("The limit to apply to the read operation"))
        .arg(Arg::new("ids")
            .long("ids")
            .action(ArgAction::Append)
            .num_args(1..)
            //.conflicts_with("filter")
            .conflicts_with_all(["filter", "limit"])
            .help("The ids to read"))
        .group(ArgGroup::new("read")
            .args(["filter", "ids"])
            .required(true));
    cmd
}

fn cmd_his_read(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("range")
            .action(ArgAction::Set)
            .num_args(1)
            .required(true)
            .help("Str encoding of a date-time range"))
        .arg(Arg::new("ids")
            .action(ArgAction::Append)
            .num_args(1..)
            .required(IsTTY::new().stdin)
            .help("Ref identifier/s of historised point"))
        .arg(Arg::new("timezone")
            .long("timezone")
            .action(ArgAction::Set)
            .num_args(1)
            .help("Timezone offset (if reading multiple points)"));
    cmd
}

fn cmd_watch_sub(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("create")
            .short('c')
            .long("create")
            .value_name("watchDis")
            //.action(ArgAction::Set)
            .num_args(1)
            .conflicts_with("watchId")
            .help("Debug/display string required when creating a new watch"))
        .arg(Arg::new("watchId")
            .short('s')
            .long("subscribe")
            //.action(ArgAction::Set)
            .num_args(1)
            .conflicts_with("create")
            .help("Str watch identifier, which is required to add entities to existing watch. If omitted, the server must open a new watch"))
        .arg(Arg::new("lease")
            .short('l')
            .long("lease")
            .action(ArgAction::Set)
            .num_args(1)
            .required(false)
            .help("Lease time in seconds"))
        .group(ArgGroup::new("read")
            .args(["create", "watchId"])
            .required(true))
        .arg(Arg::new("ids")
            .action(ArgAction::Append)
            .required(false)
            .help("The ids to add to the watch subscription"));
    cmd
}

fn cmd_watch_unsub(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("watchId")
            .required(true)
            .help("Str watch identifier"))
        .arg(Arg::new("close")
            .short('c')
            .long("close")
            .action(ArgAction::SetTrue)
            .default_missing_value("true")
            .num_args(0)
            .help("Marker tag to close the entire watch"))
        .arg(Arg::new("ids")
            .action(ArgAction::Append)
            .required(false)
            .help("The ids to remove from the watch subscription"));
    cmd
}

fn cmd_watch_poll(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("watchId")
            .required(true)
            .help("Str watch identifier"))
        .arg(Arg::new("refresh")
            .short('r')
            .long("refresh")
            .action(ArgAction::SetTrue)
            .default_missing_value("true")
            .num_args(0)
            .help("Marker tag to request full refresh"));
    cmd
}

fn cmd_his_write(op:&OP) -> Command {
    let mut cmd = cmd_generic(op);
    cmd = cmd
        .arg(Arg::new("data")
            .index(1)
            .action(ArgAction::Set)
            .required(true)
            .help("Zinc encoded history grid as described in the Haystack documentation"));
    cmd
}

fn match_auth(_: &ArgMatches) -> AnyResult<(HaystackOpTxRx,Receiver<HaystackResponse>)> {
    Ok(HaystackOpTxRx::about())
}

fn match_about(_: &ArgMatches) -> AnyResult<(HaystackOpTxRx,Receiver<HaystackResponse>)> {
    Ok(HaystackOpTxRx::about())
}

pub fn match_filetypes(matches: &ArgMatches) -> AnyResult<(HaystackOpTxRx, Receiver<HaystackResponse>)> {
    let filter = matches.get_one::<String>("filter")
        .map(|s| FStr::Str(s.as_str()));
    let limit = matches.get_one::<usize>("limit")
        .map(|v| *v);

    Ok(HaystackOpTxRx::filetypes(filter, limit)?)
}

fn match_his_read(sub_m: &ArgMatches) -> AnyResult<(HaystackOpTxRx, Receiver<HaystackResponse>)> {
    let range_opt = sub_m.get_one::<String>("range");
    let range = range_opt.map(|s| s.as_str())
        .ok_or_else(|| anyhow::anyhow!("hisRead op must have range"))?;
    let stdin_ids: String;

    let ids = match sub_m.get_many::<String>("ids") {
        Some(ids) => ids.map(|s| s.to_string()).collect(),
        None => {
            match IsTTY::new().stdin {
                true => {
                    Err(anyhow::anyhow!("hisRead op must have ids"))?
                },
                false => {
                    let mut stdin = io::stdin();
                    let mut buf: Vec<u8> = Vec::new();

                    stdin.read_to_end(&mut buf)
                        .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {}", e))?;
                    stdin_ids = String::from_utf8(buf)
                        .map_err(|e| anyhow::anyhow!("Failed to convert stdin to UTF-8 string: {}", e))?;

                    stdin_ids.split_whitespace()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()                       
                }
            }
        }   
    };
    let id_count = ids.len();

    match id_count {
        0 => {
            Err(anyhow::anyhow!("hisRead op must have ids"))?
        },
        1 => {
            let id = ids.first()
                .ok_or_else(|| anyhow::anyhow!("Failed to get id"))?;
            HaystackOpTxRx::his_read(id.as_str(),range)
                .or_else(|e| {
                    Err(anyhow::anyhow!("Failed to create hisRead op: {:?}", e))
                })
        },
        _ => {
            let timezone = sub_m.get_one::<String>("timezone").map(|s| s.as_str());
            HaystackOpTxRx::his_read_multi(ids.iter().map(|s| s.as_str()), range, timezone)
               .or_else(|e| {
                   Err(anyhow::anyhow!("Failed to create hisRead op: {:?}", e))
               })
        }
    }
}

fn match_his_write(sub_m: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let his = sub_m.get_one::<String>("data")
        .ok_or_else(|| anyhow::anyhow!("His data not provided"))?;
    HaystackOpTxRx::his_write(his.as_str())
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to create hisWrite op: {:?}", e))
        })
}

fn match_nav(sub_m: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let nav_id = sub_m.get_one::<String>("nav")
        .map(|s| s.as_str());
    HaystackOpTxRx::nav(nav_id)
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to create nav op: {:?}", e))
        })
}

fn match_ops(matches: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let filter = matches.get_one::<String>("filter")
        .map(|s| FStr::Str(s.as_str()));
    let limit = matches.get_one::<usize>("limit")
        .map(|v| *v);

    HaystackOpTxRx::ops(filter,limit)
}

fn match_defs(matches: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let filter = matches.get_one::<String>("filter")
        .map(|s| FStr::Str(s.as_str()));
    let limit = matches.get_one::<usize>("limit")
        .map(|v| *v);

    HaystackOpTxRx::defs(filter,limit)
}

fn match_libs(matches: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let filter = matches.get_one::<String>("filter")
        .map(|s| FStr::Str(s.as_str()));
    let limit = matches.get_one::<usize>("limit")
        .map(|v| *v);

    HaystackOpTxRx::libs(filter,limit)
}

fn match_read(sub_m: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    if let Some(filter) = sub_m.get_one::<String>("filter") {
        HaystackOpTxRx::read(FStr::Str(filter.as_str()), sub_m.get_one::<usize>("limit").map(|v| *v))
            .or_else(|e| {
                Err(anyhow::anyhow!("Failed to create read op: {:?}", e))
            })
    } else if let Some(ids) = sub_m.get_many::<String>("ids") {
        HaystackOpTxRx::read_by_ids(ids.map(|s| s.as_str()))
            .or_else(|e| {
                Err(anyhow::anyhow!("Failed to create read op: {:?}", e))
            })
    } else {
        Err(anyhow::anyhow!("Read op must have filter or ids"))?
    }
}

fn match_watch_poll(sub_m: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let watch_id = sub_m.get_one::<String>("watchId").map(|s| s.as_str())
        .ok_or_else(|| anyhow::anyhow!("watchId not provided"))?;
    let refresh = sub_m.get_one::<bool>("refresh")
        .map_or(false, |x| *x);
    HaystackOpTxRx::watch_poll(watch_id, refresh)
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to create watchPoll op: {:?}", e))
        })
}

fn match_watch_sub(sub_m: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let watch_dis = sub_m.get_one::<String>("create").map(|s| s.as_str());
    let watch_id = sub_m.get_one::<String>("watchId").map(|s| s.as_str());
    let lease = sub_m.get_one::<String>("lease").map(|s| s.as_str());
    let ids = sub_m.get_many::<String>("ids");
    HaystackOpTxRx::watch_sub(watch_dis, watch_id, lease, ids.map(|vr| vr.map(|s| s.as_str())))
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to create watchPoll op: {:?}", e))
        })
}

fn match_watch_unsub(sub_m: &ArgMatches) -> Result<(HaystackOpTxRx, Receiver<HaystackResponse>), Error> {
    let watch_id = sub_m.get_one::<String>("watchId").map(|s| s.as_str())
        .ok_or_else(|| anyhow::anyhow!("watchId not provided"))?;
    let close = sub_m.get_one::<bool>("close").map_or(false, |x| *x);
    let ids = sub_m.get_many::<String>("ids");
    HaystackOpTxRx::watch_unsub(watch_id, ids.map(|vr| vr.map(|s| s.as_str())), close)
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to create watchUnsub op: {:?}", e))
        })
}

const OPS: &[OP; 26] = &[
    OP {
        def:"op:about",
        doc:"Query basic information about the server. See `docHaystack::Ops#about` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:true,
        nodoc:false,
        type_name:"hx::HxAboutOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:backup",
        doc:"null",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:true,
        nodoc:true,
        type_name:"skyarcd::BackupOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:commit",
        doc:"Commit one or more diffs to the Folio database",
        is:"[op]",
        lib:"lib:hx",
        no_side_effects:false,
        nodoc:false,
        type_name:"hx::HxCommitOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:defs",
        doc:"Query the definitions in the current namespace. See `docHaystack::Ops#defs` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:true,
        nodoc:false,
        type_name:"hx::HxDefsOp",
        cmd: Some(&cmd_defs) },
    OP {
        def:"op:eval",
        doc:"Evaluate an Axon expression",
        is:"[op]",
        lib:"lib:hx",
        no_side_effects:false,
        nodoc:false,
        type_name:"hx::HxEvalOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:evalAll",
        doc:"null",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:false,
        nodoc:true,
        type_name:"skyarcd::EvalAllOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:export",
        doc:"Export a view to a file; see `docFresco::Export`",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:true,
        nodoc:false,
        type_name:"skyarcd::ExportOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:ext",
        doc:"Ext specific HTTP servicing",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:false,
        nodoc:false,
        type_name:"skyarcd::ExtOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:file",
        doc:"null",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:false,
        nodoc:true,
        type_name:"skyarcd::FileOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:filetypes",
        doc:"Query the filetype defs in the current namespace. See `docHaystack::Ops#filetypes` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:true,
        nodoc:false,
        type_name:"hx::HxFiletypesOp",
        cmd: Some(&cmd_filetypes) },
    OP {
        def:"op:funcShim",
        doc:"null",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:false,
        nodoc:true,
        type_name:"null",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:hisRead",
        doc:"Read historized time series data from a `his-point`. See `docHaystack::Ops#hisRead` chapter.",
        is:"[op]",
        lib:"lib:phIoT",
        no_side_effects:true,
        nodoc:false,
        type_name:"hx::HxHisReadOp",
        cmd: Some(&cmd_his_read) },
    OP {
        def:"op:hisWrite",
        doc:"Write historized time series data from a `his-point`. See `docHaystack::Ops#hisWrite` chapter.",
        is:"[op]",
        lib:"lib:phIoT",
        no_side_effects:false,
        nodoc:false,
        type_name:"hx::HxHisWriteOp",
        cmd: Some(&cmd_his_write) },
    OP {
        def:"op:invokeAction",
        doc:"Invoke a user action on a target entity. See `docHaystack::Ops#invokeAction` chapter.",
        is:"[op]",
        lib:"lib:phIoT",
        no_side_effects:false,
        nodoc:false,
        type_name:"null",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:io",
        doc:"null",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:false,
        nodoc:true,
        type_name:"skyarcd::IoOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:libs",
        doc:"Query the lib defs in the current namespace. See `docHaystack::Ops#libs` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:true,
        nodoc:false,
        type_name:"hx::HxLibsOp",
        cmd: Some(&cmd_libs) },
    OP {
        def:"op:link",
        doc:"null",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:true,
        nodoc:true,
        type_name:"skyarcd::LinkOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:nav",
        doc:"Query the navigation tree for discovery. See `docHaystack::Ops#nav` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:true,
        nodoc:false,
        type_name:"null",
        cmd: Some(&cmd_nav) },
    OP {
        def:"op:ops",
        doc:"Query the op defs in the current namespace. See `docHaystack::Ops#ops` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:true,
        nodoc:false,
        type_name:"hx::HxOpsOp",
        cmd: Some(&cmd_ops) },
    OP {
        def:"op:pointWrite",
        doc:"Read or command a `writable-point`. See `docHaystack::Ops#pointWrite` chapter.",
        is:"[op]",
        lib:"lib:phIoT",
        no_side_effects:false,
        nodoc:false,
        type_name:"hx::HxPointWriteOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:read",
        doc:"Query the a set of entity records by id or by filter. See `docHaystack::Ops#read` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:true,
        nodoc:false,
        type_name:"hx::HxReadOp",
        cmd: Some(&cmd_read) },
    OP {
        def:"op:rec",
        doc:"Get a record by id",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:false,
        nodoc:false,
        type_name:"skyarcd::RecOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:upload",
        doc:"null",
        is:"[op]",
        lib:"lib:skyarc",
        no_side_effects:false,
        nodoc:true,
        type_name:"skyarcd::UploadOp",
        cmd: Some(&cmd_generic) },
    OP {
        def:"op:watchPoll",
        doc:"Poll a watch subscription. See `docHaystack::Ops#watchPoll` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:false,
        nodoc:false,
        type_name:"null",
        cmd: Some(&cmd_watch_poll) },
    OP {
        def:"op:watchSub",
        doc:"Subscribe to entity data. See `docHaystack::Ops#watchSub` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:false,
        nodoc:false,
        type_name:"null",
        cmd: Some(&cmd_watch_sub) },
    OP {
        def:"op:watchUnsub",
        doc:"Unsubscribe to entity data. See `docHaystack::Ops#watchUnsub` chapter.",
        is:"[op]",
        lib:"lib:ph",
        no_side_effects:false,
        nodoc:false,
        type_name:"null",
        cmd: Some(&cmd_watch_unsub) },
];

#[derive(Clone, Debug)]
pub enum Destination {
    Url(Url),
    Host {
        username: Option<String>,
        host: Host,
    },
    Env {
        url: Url,
        username: String,
        password: String,
        accept_invalid_certs: bool,
        auth_info: Option<String>,
    }
}

fn parse_destination(input: &str) -> Result<Destination, String> {
    if let Ok(url) = Url::parse(input) {
        return Ok(Destination::Url(url));
    }

    let parts: Vec<&str> = input.split('@').collect();
    if parts.len() == 2 {
        // Extract username and host
        let username = Some(parts[0].to_string());
        if let Ok(host) = Host::parse(parts[1]) {
            return Ok(Destination::Host { username, host });
        }
    } else if parts.len() == 1 {
        // No username, only host
        if let Ok(host) = Host::parse(parts[0]) {
            return Ok(Destination::Host { username: None, host });
        }
    }

    Err(format!("Invalid destination format: {}", input))
}

pub fn cli(is_tty: IsTTY) -> Command {
    let mut cmd = command!()
    .arg(Arg::new("destination")
        .action(ArgAction::Set)
        //.global(true)
        .value_parser(parse_destination)
        .required(false)
        .help("The haystack server destination which can be specified as either [user@]hostname or a URI of the form https://[user@]hostname[:port]."))
    .arg(Arg::new("username")
        .short('u')
        .long("username")
        .action(ArgAction::Set)
        .num_args(1)
        .help("Username"))
    .arg(Arg::new("password")
        .short('p')
        .long("password")
        .action(ArgAction::Set)
        .num_args(1)
        .help("Password"))
    .arg(Arg::new("accept-invalid-certs")
        //.short('')
        .long("accept-invalid-certs")
        //.action(ArgAction::SetTrue)
        .value_parser(value_parser!(bool))
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true")
        .help("Tell the client to accept invalid SSL certificates"));

        cmd = cmd
            .subcommand(Command::new("auth")
                .about("Return the auth information. Normally stored for reuse in environment variable 'HAYSTACK_AUTH_CONFIG'"))
            .subcommand(Command::new("repl")
                .about("Run the REPL"));

    for op in OPS {
        if let Some(op_cmd) = op.cmd {
            cmd = cmd.subcommand(op_cmd(op));
        }
    }
    cmd
}

async fn repl_generic<'a, T:NumTrait>(m_func: MATCH_FUNC_TYPE, matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    let Context { abort_handle, sender: client } = context;
    
    let (op, resp) = m_func(&matches)
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to create haystack op: {:?}", e))
        })?;

    let response = send_haystack_op::<T>(client, resp, op).await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to get response: {:?}", e))
        })?;

    Ok(Some(response.get_raw()))
}

async fn repl_not_implemented<'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    todo!("Not implemented yet");
}

async fn repl_auth<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_auth, matches, context).await
}

async fn repl_about<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_about, matches, context).await
}

async fn repl_ops<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_ops, matches, context).await
}

async fn repl_filetypes<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_filetypes, matches, context).await
}

async fn repl_nav<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_nav, matches, context).await
}

async fn repl_defs<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_defs, matches, context).await
}

async fn repl_libs<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_libs, matches, context).await
}

async fn repl_read<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_read, matches, context).await
}

async fn repl_his_read<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_his_read, matches, context).await
}

async fn repl_watch_sub<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_watch_sub, matches, context).await
}

async fn repl_watch_unsub<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_watch_unsub, matches, context).await
}

async fn repl_watch_poll<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_watch_poll, matches, context).await
}

async fn repl_watch_his_write<'a, T:NumTrait + 'a>(matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    repl_generic::<T>(match_his_write, matches, context).await
}

async fn update_prompt<T>(_context: &mut T) -> AnyResult<Option<String>> {
    Ok(Some("updated".to_string()))
}

pub fn repl<'a, T:NumTrait + 'a>(client: &'a mut Sender<HaystackOpTxRx>, abort_handle: &'a AbortHandle, history_file: PathBuf) -> Repl<Context<'a>, anyhow::Error> {
    let context: Context<'_> = Context {
        abort_handle,
        sender: client,
    };
    
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let mut repl_obj = Repl::new(context)
        .with_name(PKG_NAME)
        .with_version(VERSION)
        .with_history(history_file, 1000)
        .with_quick_completions(true)
        .with_partial_completions(true)
        .with_prompt("hs");

    fn get_cmd<'a>(key: &str) -> Command {
        let op = OPS.iter().find(|x| x.def == key).unwrap();
        op.cmd.unwrap()(op)
    }

    repl_obj = repl_obj
        .with_command_async(get_cmd("op:about"), |args, context| Box::pin(repl_auth::<T>(args, context)))
        .with_command_async(get_cmd("op:about"), |args, context| Box::pin(repl_about::<T>(args, context)))
        .with_command_async(get_cmd("op:backup"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:commit"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:defs"), |args, context| Box::pin(repl_defs::<T>(args, context)))
        .with_command_async(get_cmd("op:eval"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:evalAll"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:export"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:ext"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:file"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:filetypes"), |args, context| Box::pin(repl_filetypes::<T>(args, context)))
        .with_command_async(get_cmd("op:funcShim"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:hisRead"), |args, context| Box::pin(repl_his_read::<T>(args, context)))
        .with_command_async(get_cmd("op:hisWrite"), |args, context| Box::pin(repl_watch_his_write::<T>(args, context)))
        .with_command_async(get_cmd("op:invokeAction"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:io"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:libs"), |args, context| Box::pin(repl_libs::<T>(args, context)))
        .with_command_async(get_cmd("op:link"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:nav"), |args, context| Box::pin(repl_nav::<T>(args, context)))
        .with_command_async(get_cmd("op:ops"), |args, context| Box::pin(repl_ops::<T>(args, context)))
        .with_command_async(get_cmd("op:pointWrite"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:read"), |args, context| Box::pin(repl_read::<T>(args, context)))
        .with_command_async(get_cmd("op:rec"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:upload"), |args, context| Box::pin(repl_not_implemented(args, context)))
        .with_command_async(get_cmd("op:watchPoll"), |args, context| Box::pin(repl_watch_poll::<T>(args, context)))
        .with_command_async(get_cmd("op:watchSub"), |args, context| Box::pin(repl_watch_sub::<T>(args, context)))
        .with_command_async(get_cmd("op:watchUnsub"), |args, context| Box::pin(repl_watch_unsub::<T>(args, context)));
    repl_obj
}

pub struct Context<'a> {
    abort_handle: &'a AbortHandle,
    sender: &'a Sender<HaystackOpTxRx>,
}

pub async fn eval_subcommand<'a, T:NumTrait>(get_op: MATCH_FUNC_TYPE, matches: ArgMatches, context: &mut Context<'a>) -> AnyResult<Option<String>> {
    let Context { abort_handle, sender: client } = context;
    
    let (op, resp) = get_op(&matches)
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to create haystack op: {:?}", e))
        })?;

    let response = send_haystack_op::<T>(client, resp, op).await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to get response: {:?}", e))
        })?;
    
    Ok(Some(response.get_raw()))
}

pub async fn send_haystack_op<T: NumTrait>(client: &Sender<HaystackOpTxRx>, resp: Receiver<HaystackResponse>, op: HaystackOpTxRx) -> AnyResult<HaystackResponse> {
    client.send(op).await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to send request: {:?}", e))
        })?;
    
    resp.await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to get response: {:?}", e))
        })?.as_result::<T>()
}

pub fn get_haystack_op(cmd: &str, matches: &ArgMatches) -> Result<(HaystackOpTxRx,Receiver<HaystackResponse>), Error> {
    let res = match (cmd, matches) {
        ("authInfo", sub_m) => match_auth(sub_m),
        ("about", sub_m) => match_about(sub_m),
        // TODO: Implement close
        ("defs", sub_m) => match_defs(sub_m),
        ("libs", sub_m) => match_libs(sub_m),
        ("ops", sub_m) => match_ops(sub_m),
        ("filetypes", sub_m,) => match_filetypes(sub_m),
        ("nav", sub_m) => match_nav(sub_m),
        ("read", sub_m) => match_read(sub_m),
        ("hisRead", sub_m) => match_his_read(sub_m),
        ("watchSub", sub_m) => match_watch_sub(sub_m),
        ("watchUnsub", sub_m) => match_watch_unsub(sub_m),
        ("watchPoll", sub_m) => match_watch_poll(sub_m),
        ("hisWrite", sub_m) => match_his_write(sub_m),
        _ => {
            return Err(anyhow::anyhow!("Subcommand \"{}\" either not supported or doesn't exist", matches.subcommand().ok_or_else(|| anyhow::anyhow!("No subcommand provided"))?.0))
        }
    };

    res
}