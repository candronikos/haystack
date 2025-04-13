use clap::{Arg, ArgAction, ArgGroup, Command, Parser, Subcommand};
use url::{Url, Host};

/*
ops().toRecList.map(r => "OP { def:\""+r->def+"\",doc:\""+r["doc"]+"\",is:\""+r->is+"\",lib:\""+r->lib+"\",no_side_effects:\""+r.has("no_side_effects")+"\",nodoc:\""+r.has("nodoc")+"\",type_name:\""+r["type_name"]+"\" }") // def,doc,is,lib,no_side_effects,nodoc,type_name
*/
pub struct OP<'a:'static> {
    def: &'a str,
    doc: &'a str,
    is: &'a str,
    lib: &'a str,
    no_side_effects: bool,
    nodoc: bool,
    type_name: &'a str,
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
            .required(true)
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

const OPS: &[OP<'static>; 26] = &[
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
        cmd: Some(&cmd_generic) },
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
        cmd: Some(&cmd_generic) },
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
        cmd: Some(&cmd_generic) },
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
        cmd: Some(&cmd_generic) },
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
        cmd: Some(&cmd_generic) },
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

pub fn cli() -> Command {
    let mut cmd = command!()
    .arg(Arg::new("destination")
        .action(ArgAction::Set)
        //.global(true)
        .value_parser(parse_destination)
        .required(true)
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

    for op in OPS {
        if let Some(op_cmd) = op.cmd {
            cmd = cmd.subcommand(op_cmd(op));
        }
    }
    cmd
}