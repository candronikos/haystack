#[macro_use]
extern crate clap;

use url::Url;

use std::sync::Arc;
use tokio::sync::Mutex;

use haystackclientlib::{HSession, ops::HaystackOpTxRx};

use anyhow::{Context, Error, Result as AnyResult, anyhow};
use dialoguer::Confirm;
use dirs::config_dir;
use std::io::Read;
use std::path::Path;

use saphyr::{LoadableYamlNode, Yaml, YamlEmitter};

const CONFIG_DIR_NAME: &str = "haystack";
const CONFIG_FILE_NAME: &str = "config.yaml";
const HISTORY_FILE_NAME: &str = "history.txt";

mod args;
use args::{Destination, IsTTY, cli, get_haystack_op, repl, send_haystack_op};

type NUMBER = f64;

#[derive(Debug)]
struct ConnInfo {
    username: String,
    password: String,
    url: Url,
    accept_invalid_certs: bool,
    bearer: Option<String>,
}

fn get_credentials(
    args: &clap::ArgMatches,
    config: &Option<Vec<Yaml>>,
    env_dest: Option<Destination>,
) -> AnyResult<ConnInfo, Error> {
    let destination: Option<&Destination> = args.get_one("destination").or(env_dest.as_ref());
    let arg_user: Option<String> = args.get_one::<&str>("username").map(|s| s.to_string());
    let arg_pass: Option<String> = args.get_one::<&str>("password").map(|s| s.to_string());
    let arg_accept_invalid_certs: Option<bool> =
        args.get_one::<bool>("accept-invalid-certs").map(|s| *s);

    let cur_config = match config {
        Some(config) => {
            if let Some(destination) = destination {
                match destination {
                    Destination::Url(url) => Ok(ConnInfo {
                        username: arg_user.ok_or(anyhow::anyhow!("Username not provided"))?,
                        password: arg_pass.ok_or(anyhow::anyhow!("Password not provided"))?,
                        accept_invalid_certs: arg_accept_invalid_certs.ok_or(anyhow!(
                            "Should never happen. Unable to resolve for 'arg_accept_invalid_certs'"
                        ))?,
                        url: url.clone(),
                        bearer: None,
                    }),
                    Destination::Host {
                        username: host_user,
                        host,
                    } => {
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
                    Destination::Env {
                        url,
                        username,
                        password,
                        accept_invalid_certs,
                        ..
                    } => {
                        let url = url.clone();
                        let username = arg_user
                            .or_else(|| Some(username.clone()))
                            .ok_or_else(|| anyhow::anyhow!("Unable to resolve for username"))?;
                        let password = arg_pass
                            .or_else(|| Some(password.clone()))
                            .ok_or_else(|| anyhow::anyhow!("Password not provided"))?;
                        let accept_invalid_certs = arg_accept_invalid_certs
                            .or_else(|| Some(*accept_invalid_certs))
                            .ok_or_else(|| anyhow::anyhow!("Should never happen. Unable to resolve for 'accept_invalid_certs'"))?;
                        Ok(ConnInfo {
                            username,
                            password,
                            url,
                            accept_invalid_certs,
                            bearer: None,
                        })
                    }
                }
            } else {
                return Err(anyhow::anyhow!("Destination not provided"));
            }
        }
        None => destination
            .ok_or_else(|| anyhow::anyhow!("Destination not provided"))
            .and_then(|dest| match dest {
                Destination::Url(url) => Ok(ConnInfo {
                    username: arg_user.ok_or(anyhow::anyhow!("Username not provided"))?,
                    password: arg_pass.ok_or(anyhow::anyhow!("Password not provided"))?,
                    url: url.clone(),
                    accept_invalid_certs: arg_accept_invalid_certs.ok_or(anyhow!(
                        "Should never happen. Unable to resolve for 'arg_accept_invalid_certs'"
                    ))?,
                    bearer: None,
                }),
                _ => Err(anyhow::anyhow!("Config file does not exist!")),
            }),
    };
    cur_config
}

#[tokio::main]
async fn main() -> AnyResult<()> {
    let is_tty = IsTTY::new();

    let env_config_opt: Option<String> = match std::env::var("HAYSTACK_AUTH_CONFIG") {
        Ok(session) => Some(session),
        Err(_) => None,
    };

    let matches = cli(is_tty).get_matches();
    // TODO: Handle situations where config_dir doesn't find a directory
    let user_config_dir = config_dir().context("Config directory error")?;
    let hs_config_dir = Path::join(&user_config_dir, CONFIG_DIR_NAME);
    let hs_config_file = Path::join(&hs_config_dir, CONFIG_FILE_NAME);
    let history_file = Path::join(&hs_config_dir, HISTORY_FILE_NAME);

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
            if should_exit_program {
                return Result::Ok(());
            }
        }
    }

    let mut config: Option<Vec<Yaml>> = None;
    if hs_config_file.exists() {
        let mut hs_config_file_handle = std::fs::File::open(hs_config_file)?;
        let mut hs_config_file_string = String::new();
        hs_config_file_handle.read_to_string(&mut hs_config_file_string)?;
        //let mut config: Vec<Yaml> = Vec::new();
        config = Some(Yaml::load_from_str(&hs_config_file_string)?);
    };

    let mut env_config: Option<Destination> = None;
    if let Some(s) = env_config_opt {
        let e_conf_vec = Yaml::load_from_str(&s)?;
        let e_conf = e_conf_vec
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("Failed to get env config"))?;
        let url = e_conf["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get URL from env config"))?;
        let username = e_conf["username"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get username from env config"))?;
        let password = e_conf["password"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get password from env config"))?;
        let accept_invalid_certs = e_conf["accept-invalid-certs"]
            .as_bool()
            .or(Some(false))
            .ok_or_else(|| {
                anyhow::anyhow!("Should never happen. Unable to resolve for 'accept_invalid_certs'")
            })?;
        let auth_info = e_conf["auth-info"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get auth-info from env config"))?;

        env_config = Some(Destination::Env {
            url: Url::parse(url)?,
            username: username.to_string(),
            password: password.to_string(),
            accept_invalid_certs: accept_invalid_certs,
            auth_info: Some(auth_info.to_string()),
        });
    }

    let conn_info = get_credentials(&matches, &config, env_config)?;

    let (abort_client, mut client, auth_token) = HSession::new(
        conn_info.url.to_string(),
        conn_info.username.to_owned(),
        conn_info.password.to_owned(),
        conn_info.accept_invalid_certs.to_owned(),
        Arc::new(Mutex::new(None)),
        None,
    )
    .await
    .or_else(|e| Err(anyhow::anyhow!("Failed to create HSession: {:?}", e)))?;

    match &matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse subcommands"))?
    {
        ("repl", _) => {
            let _ = repl::<NUMBER>(&mut client, &abort_client, history_file)
                .run_async()
                .await;

            let (_close_op, _close_resp) = HaystackOpTxRx::close();
        }
        ("auth", _) => {
            let mut conf_yaml = saphyr::Mapping::new();
            //conf_yaml.insert("name".to_string(), conn_info.bearer.unwrap_or_default());
            conf_yaml.insert(
                Yaml::value_from_str("url"),
                Yaml::Value(saphyr::Scalar::String(std::borrow::Cow::Owned(
                    (&conn_info).url.clone().to_string(),
                ))),
            );
            conf_yaml.insert(
                Yaml::value_from_str("username"),
                Yaml::Value(saphyr::Scalar::String(std::borrow::Cow::Owned(
                    (&conn_info).username.clone(),
                ))),
            );
            conf_yaml.insert(
                Yaml::value_from_str("password"),
                Yaml::Value(saphyr::Scalar::String(std::borrow::Cow::Owned(
                    (&conn_info).password.clone(),
                ))),
            );
            conf_yaml.insert(
                Yaml::value_from_str("accept-invalid-certs"),
                Yaml::Value(saphyr::Scalar::Boolean((&conn_info).accept_invalid_certs)),
            );
            if let Some(bearer) = auth_token {
                conf_yaml.insert(
                    Yaml::value_from_str("auth-info"),
                    Yaml::Value(saphyr::Scalar::String(std::borrow::Cow::Owned(bearer))),
                );
            } else {
                Err(anyhow::anyhow!("Failed to get auth token"))?;
            }
            let mut buffer = String::new();
            YamlEmitter::new(&mut buffer)
                .dump(&Yaml::Mapping(conf_yaml))
                .or_else(|e| Err(anyhow::anyhow!("Failed to write YAML: {:?}", e)))?;
            println!("{}", buffer);
            return Ok(());
        }
        (cmd, sub_m) => {
            let (op, resp) = get_haystack_op(*cmd, *sub_m) //(&matches)
                .or_else(|e| {
                    Err(anyhow::anyhow!("Failed to get haystack op: {:?}", e))
                })?;

            let response = send_haystack_op::<NUMBER>(&mut client, resp, op)
                .await?
                .as_result::<NUMBER>()?;

            print!("{}", response.get_raw());
        }
    };

    let (close_op, close_resp) = HaystackOpTxRx::close();
    client
        .send(close_op)
        .await
        .or_else(|e| Err(anyhow::anyhow!("Failed to send close request: {:?}", e)))?;
    let _ = close_resp.await?.as_result::<NUMBER>()?;

    // TODO: Check if the response is an error
    Result::Ok(())
}
