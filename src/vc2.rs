use clap::ArgMatches;
use clap::App;
use clap::SubCommand;
use futures::{future};
use tokio;
use tokio_core;
use hyper::{Method, Client, Body, Request};
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
    //let request = Request::builder().uri(endpoint).header("API-Key", token).body(Body::empty()).unwrap();
    let mut request:Request<Body> = Request::new(Method::Get, "http://google.com".parse().unwrap());
    let mut headers = request.headers_mut().append_raw("API-Key", token);

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);
    let job = client.request(request);
    core.run(job).unwrap();

    /*
    rt::run(
        client.request(request)
            .map(move |res| {
                println!("status: {}", res.status());
                if let Ok(v) = res.into_body().map(|ch| ch.into_bytes()).collect().wait() {
                    let mut response = v.iter().fold("".to_string(), |acc, x| format!("{}{:?}", acc, x));
                    println!("{}", &response);
                };
            })
            .map_err(|err| {
                println!("{}", err)
            })
    );
    */

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
