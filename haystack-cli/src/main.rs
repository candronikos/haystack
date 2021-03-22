#[macro_use]
extern crate clap;
use clap::App;

use futures::future::{Abortable, AbortHandle};
use tokio::sync::mpsc;

use haystack_client::{HSession,ops::HaystackOp};

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let (abort_client, client): (AbortHandle,mpsc::Sender<HaystackOp>) = HSession::new(
        "http://host.docker.internal:8080/api/demo/".to_owned(),
        "user".to_owned(),
        "user".to_owned(),
        None
    ).unwrap();

    let (op,resp) = match matches.subcommand() {
        ("about", Some(_)) => HaystackOp::about(),
        ("ops", Some(_)) => HaystackOp::ops(),
        ("formats", Some(_)) => HaystackOp::formats(),
        ("read", Some(sub_m)) => {
            let filter_opt = sub_m.value_of("filter");
            match filter_opt {
                Some(filter) => HaystackOp::read(filter.to_owned(), None).unwrap(),
                None => panic!("Read op must have filter")
            }
        },
        ("readIds", Some(sub_m)) => {
            let ids_opt = sub_m.values_of("ids");
            match ids_opt {
                Some(ids) => HaystackOp::read_by_ids(ids.collect::<Vec<&str>>().join("\n")).unwrap(),
                None => panic!("Read op must have ids")
            }
        },
        _ => panic!("Doesn't match any available subcommand")
    };

    let res = client.send(op).await;

    if let Err(e) = res {
        panic!("Failed to send request");
    }

    match resp.await {
        Ok(response) => print!("{}",response.get_raw()),
        Err(e) => panic!("Failed to receive response")
    };
}
