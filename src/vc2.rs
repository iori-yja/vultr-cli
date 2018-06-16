use clap::ArgMatches;
use clap::App;
use clap::SubCommand;
use futures::{future, Future};
use tokio;
use hyper::{Client, Body, Request};
use hyper_tls::HttpsConnector;

pub fn cli<'a>() -> App<'a,'a> {
    SubCommand::with_name("vc2")
        .about("Vultr Could Compute")
        .subcommand(SubCommand::with_name("run-instances"))
        .subcommand(SubCommand::with_name("describe-instances"))
        .subcommand(SubCommand::with_name("start-instances"))
        .subcommand(SubCommand::with_name("stop-instances"))
        .subcommand(SubCommand::with_name("terminate-instances"))
        .subcommand(SubCommand::with_name("reboot-instances"))
}

fn call_api(token: &str, endpoint: &str) -> Result<String, String> {
    let mut https = HttpsConnector::new(4).unwrap();
    https.force_https(true);
    let client = Client::builder().build::<_, Body>(https);
    let mut request = Request::builder();
    let request = request.uri(endpoint).header("API-Key", token).body(Body::empty()).unwrap();

    client.request(request);
    panic!();

}

fn run(args: Option<&ArgMatches>) {
    let conf = super::read_config();
}

fn start(args: Option<&ArgMatches>) {
    let conf = super::read_config();
}

fn stop(args: Option<&ArgMatches>) {
    let conf = super::read_config();
}

fn terminate(args: Option<&ArgMatches>) {
    let conf = super::read_config();
}

fn reboot(args: Option<&ArgMatches>) {
    let conf = super::read_config();
}

fn describe(args: Option<&ArgMatches>) {
    let conf = super::read_config();
}

pub fn handle(args: Option<&ArgMatches>) {
    match args.unwrap().subcommand() {
        ("run-instances", arg) => run(arg),
        ("start-instances", arg) => start(arg),
        ("stop-instances", arg) => stop(arg),
        ("terminate-instances", arg) => terminate(arg),
        ("reboot-instances", arg) => reboot(arg),
        ("describe-instances", arg) => describe(arg),
        (_,_) => panic!(),
    }
}
