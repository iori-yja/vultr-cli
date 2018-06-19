use clap::ArgMatches;
use clap::App;
use clap::SubCommand;
use tokio;
use tokio_core;
use hyper::{Method, Client, Body, Request};
use hyper_tls::HttpsConnector;
use tokio::prelude::Stream;

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
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let mut https = HttpsConnector::new(4, &handle).unwrap();
    let client = Client::configure().connector(https).build(&handle);

    let mut request:Request<Body> = Request::new(Method::Get, "https://google.com".parse().unwrap());
    let mut headers = request.headers_mut().append_raw("API-Key", token);

    let job = client.request(request);
    let response_body = core.run(job).unwrap().body();

    response_body.map(|ch| {
        print!("{}", String::from_utf8(ch.to_vec()).unwrap());
    }).poll();

    Ok("".to_string())
}

fn run(args: Option<&ArgMatches>) {
    //let conf = super::read_config();
    call_api("", "https://google.com");
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
