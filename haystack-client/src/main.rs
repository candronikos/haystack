#[macro_use]
extern crate clap;
//use clap::App;
use clap::Parser;

use futures::future::{Abortable, AbortHandle};
use haystackclientlib::ops::FStr;
use tokio::sync::mpsc;
use url::Url;

use std::process::exit;
use std::sync::{Arc};
use tokio::sync::Mutex;

use haystackclientlib::{HSession,ops::HaystackOpTxRx};

use dirs::config_dir;
use std::fs::{create_dir};
use std::io::{self, Read, Write};
use std::path::Path;
use dialoguer::{Confirm, Result as DialoguerResult};
use anyhow::{anyhow, Context, Error, Result as AnyResult};

use saphyr::{LoadableYamlNode, Yaml, YamlLoader};

const CONFIG_DIR_NAME: &str = "haystack";
const CONFIG_FILE_NAME: &str = "config.yaml";

mod args;
use args::{cli, Destination};

#[derive(Debug)]
struct ConnInfo {
    username: String,
    password: String,
    url: Url,
    accept_invalid_certs: bool,
    bearer: Option<String>
}

struct SessionConf {
    try_reuse: bool,
    update_cache: bool
}

struct Settings {
    conn: Option<ConnInfo>,
    session: SessionConf,
}

impl Settings {
    fn new(conn: Option<ConnInfo>, session: SessionConf) -> Self {
        Self { conn, session }
    }

    fn default() -> Self {
        let conn:Option<ConnInfo> = None;
        let session: SessionConf = SessionConf {
            try_reuse: true,
            update_cache: true,
        };
        Self { conn, session }
    }

    fn update_from_yaml(&self, yaml: Vec<Yaml>) -> Result<(),Error> {
        let conf = yaml[0].as_mapping()
            .ok_or_else(|| anyhow::anyhow!("Failed to get hash from config.yaml"))?;
        conf;
        Ok(())
    }
}

fn get_credentials(args: &clap::ArgMatches, config: &Option<Vec<Yaml>>) -> AnyResult<ConnInfo,Error> {
    //let sub_cmd = args.subcommand_matches(args.subcommand_name().ok_or(anyhow::anyhow!("Failed to get subcommand name"))?)
    //    .ok_or_else(|| anyhow::anyhow!("Failed to get subcommand matches"))?;

    let destination: Option<&Destination> = args.get_one("destination");
    let arg_user: Option<String> = args.get_one::<&str>("username").map(|s| s.to_string());
    let arg_pass: Option<String> = args.get_one::<&str>("password").map(|s| s.to_string());
    let arg_accept_invalid_certs: Option<bool> = args.get_one::<bool>("accept-invalid-certs").map(|s| *s);

    let cur_config = match config {
        Some(config) => {
            if let Some(destination) = destination {
                match destination {
                    Destination::Url(url) => Ok(ConnInfo {
                            username: arg_user.ok_or(anyhow::anyhow!("Username not provided"))?,
                            password: arg_pass.ok_or(anyhow::anyhow!("Password not provided"))?,
                            accept_invalid_certs: arg_accept_invalid_certs.ok_or(anyhow!("Should never happen. Unable to resolve for 'arg_accept_invalid_certs'"))?,
                            url: url.clone(),
                            bearer: None,
                        }),
                    Destination::Host { username: host_user, host } => {
                        config.iter()
                        .find(|e| {
                            e["name"].as_str().unwrap() == host.to_string().as_str()
                        })
                        .ok_or_else(|| anyhow::anyhow!("Failed to find host in config file"))
                        .and_then(|e: &Yaml| -> AnyResult<ConnInfo,Error> {
                            let username = arg_user
                                .or(host_user.clone())
                                .or(e["username"].as_str().map(|s| s.to_string()))
                                .ok_or_else(|| anyhow::anyhow!("Unable to resolve for username"))?;
                            let password = arg_pass
                                .or(e["password"].as_str().map(|s| s.to_string()))
                                .ok_or_else(|| anyhow::anyhow!("Password not provided"))?;
                            let accept_invalid_certs = arg_accept_invalid_certs
                                .or(e["accept-invalid-certs"].as_bool())
                                .ok_or_else(|| anyhow::anyhow!("Should never happen. Unable to resolve for 'accept_invalid_certs'"))?;
                            let url_str = e["url"].as_str().ok_or_else(|| anyhow::anyhow!("Failed to get URL from config file"))?;
                            let url = Url::parse(url_str)?;//.or_else(|_| anyhow::anyhow!("Failed to parse URL from config file"))?;
                            Ok(ConnInfo { username, password, url, accept_invalid_certs, bearer: None, })
                        })
                    }
                }
            } else {
                return Err(anyhow::anyhow!("Destination not provided"));
            }
        },
        None => {
            destination
            .ok_or_else(|| anyhow::anyhow!("Destination not provided"))
            .and_then(|dest| {
                match dest {
                    Destination::Url(url) => Ok(ConnInfo {
                            username: arg_user.ok_or(anyhow::anyhow!("Username not provided"))?,
                            password: arg_pass.ok_or(anyhow::anyhow!("Password not provided"))?,
                            url: url.clone(),
                            accept_invalid_certs: arg_accept_invalid_certs.ok_or(anyhow!("Should never happen. Unable to resolve for 'arg_accept_invalid_certs'"))?,
                            bearer: None,
                        }),
                    Destination::Host { username: host_user, host } => {
                        Err(anyhow::anyhow!("Config file does not exist!"))
                    }
                }
            })
        },
    };
    cur_config
}

#[tokio::main]
async fn main() -> AnyResult<(),Error> {
    let matches = cli().get_matches();
    // TODO: Handle situations where config_dir doesn't find a directory
    let user_config_dir = config_dir().context("Config directory error")?;
    let hs_config_dir = Path::join(&user_config_dir, CONFIG_DIR_NAME);
    let hs_config_file = Path::join(&hs_config_dir, CONFIG_FILE_NAME);
    let mut haystack_settings = Settings::default();

    if !hs_config_dir.exists() {
        let create_config_dir = Confirm::new()
            .with_prompt("Do you want to create it and associated config files?")
            .default(false)
            .show_default(true)
            .wait_for_newline(true)
            .interact()?;

        if create_config_dir {
            std::fs::create_dir_all(hs_config_dir)?;
        } else {
            let should_exit_program = Confirm::new()
                .with_prompt("Exit?")
                .default(false)
                .show_default(true)
                .wait_for_newline(true)
                .interact()?;
            if should_exit_program { return Result::Ok(()) }
        }
    }

    let config: Option<Vec<Yaml>>;
    if hs_config_file.exists() {
        let mut hs_config_file_handle = std::fs::File::open(hs_config_file)?;
        let mut hs_config_file_string = String::new();
        hs_config_file_handle.read_to_string(&mut hs_config_file_string)?;
        //let mut config: Vec<Yaml> = Vec::new();
        config = Some(
            Yaml::load_from_str(&hs_config_file_string)?
        );
    } else {
        config = None;
    }
    let conn_info = get_credentials(&matches, &config)?;

    let (abort_client, client) = HSession::new(
        conn_info.url.to_string(),
        conn_info.username,
        conn_info.password,
        conn_info.accept_invalid_certs,
        Arc::new(Mutex::new(None)),
        None
    ).or_else(|e| {
        Err(anyhow::anyhow!("Failed to create HSession: {:?}", e))
    })?;

    let (op, resp) = match matches.subcommand().ok_or_else(|| anyhow::anyhow!("No subcommand provided"))? {
        ("about", _) => HaystackOpTxRx::about(),
        ("ops", _) => HaystackOpTxRx::ops(),
        ("filetypes", _) => HaystackOpTxRx::filetypes(),
        ("nav", sub_m) => {
            let nav_id = sub_m.get_one::<String>("nav")
                .map(|s| s.as_str());
            HaystackOpTxRx::nav(nav_id)
                .or_else(|e| {
                    Err(anyhow::anyhow!("Failed to create nav op: {:?}", e))
                })?
        },
        ("read", sub_m) => {
            if let Some(filter) = sub_m.get_one::<String>("filter") {
                HaystackOpTxRx::read(FStr::Str(filter.as_str()), sub_m.get_one::<usize>("limit").map(|v| *v))
                    .or_else(|e| {
                        Err(anyhow::anyhow!("Failed to create read op: {:?}", e))
                    })?
            } else if let Some(ids) = sub_m.get_many::<String>("ids") {
                HaystackOpTxRx::read_by_ids(ids.map(|s| s.as_str()))
                    .or_else(|e| {
                        Err(anyhow::anyhow!("Failed to create read op: {:?}", e))
                    })?
            } else {
                Err(anyhow::anyhow!("Read op must have filter or ids"))?
            }
        },
        ("hisRead", sub_m) => {
            let range_opt = sub_m.get_one::<String>("range");
            let range = range_opt.map(|s| s.as_str())
                .ok_or_else(|| anyhow::anyhow!("hisRead op must have range"))?;
            let id_count = sub_m.get_many::<String>("ids")
                .ok_or_else(|| anyhow::anyhow!("Failed to get ids"))?
                .count();
            match id_count {
                0 => {
                    Err(anyhow::anyhow!("hisRead op must have ids"))?
                },
                1 => {
                    let id = sub_m.get_one::<String>("ids")
                        .ok_or_else(|| anyhow::anyhow!("Failed to get id"))?;
                    HaystackOpTxRx::his_read(id.as_str(),range)
                        .or_else(|e| {
                            Err(anyhow::anyhow!("Failed to create hisRead op: {:?}", e))
                        })?
                },
                _ => {
                    let ids = sub_m.get_many::<String>("ids")
                       .ok_or_else(|| anyhow::anyhow!("Failed to get ids"))?;
                    let timezone = sub_m.get_one::<String>("timezone").map(|s| s.as_str());
                    HaystackOpTxRx::his_read_multi(ids.map(|s| s.as_str()), range, timezone)
                       .or_else(|e| {
                           Err(anyhow::anyhow!("Failed to create hisRead op: {:?}", e))
                       })?
                }
            }
        },
        ("watchSub", sub_m) => {
            let watch_dis = sub_m.get_one::<String>("create").map(|s| s.as_str());
            let watch_id = sub_m.get_one::<String>("watchId").map(|s| s.as_str());
            let lease = sub_m.get_one::<String>("lease").map(|s| s.as_str());
            let ids = sub_m.get_many::<String>("ids");
            HaystackOpTxRx::watch_sub(watch_dis, watch_id, lease, ids.map(|vr| vr.map(|s| s.as_str())))
                .or_else(|e| {
                    Err(anyhow::anyhow!("Failed to create watchPoll op: {:?}", e))
                })?
        },
        _ => {
            return Err(anyhow::anyhow!("Subcommand \"{}\" either not supported or doesn't exist", matches.subcommand().ok_or_else(|| anyhow::anyhow!("No subcommand provided"))?.0))
        }
    };

    client.send(op).await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to send request: {:?}", e))
        })?;
    
    let response = resp.await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to get response: {:?}", e))
        })?;
    print!("{}", response.get_raw());
    io::stdout().flush().unwrap();

    let (close_op, close_resp) = HaystackOpTxRx::close();
    client.send(close_op).await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to send close request: {:?}", e))
        })?;
    close_resp.await
        .or_else(|e| {
            Err(anyhow::anyhow!("Failed to get close response: {:?}", e))
        })?;
    Result::Ok(())
}

