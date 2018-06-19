extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate tokio;
extern crate tokio_core;
extern crate toml;

mod vc2;

use clap::App;
use clap::SubCommand;
use std::fs::File;
use std::io::{self, Read, Write};

static DEF_SERVER: &str = "https://api.vultr.com";

#[derive(Deserialize, Serialize)]
pub struct Config {
    access_token: String,
    default_region: String,
    default_output: String,
    api_server: String,
}

impl Config {
    pub fn new(token: String, region: String, output: String) -> Config {
        Config {
            access_token: token,
            default_region: region,
            default_output: output,
            api_server: DEF_SERVER.to_string(),
        }
    }
}

fn read_dialog(msg: &str, def: &str) -> String {
    print!("{}[{}]:", msg, def);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let len = io::stdin().read_line(&mut input);

    if len.is_err() {
        panic!();
    }

    if let Ok(0) = len {
        def.to_string()
    } else {
        input.truncate(len.unwrap() - 1);
        input
    }
}

pub fn read_config() -> Config {
    let mut input = String::new();
    match File::open(".vultr").and_then(|mut f| f.read_to_string(&mut input)) {
        Ok(_) => match toml::from_str(&input) {
            Ok(conf) => conf,
            Err(_) => {
                println!("Configuration file is broken.\n{}", input);
                configure();
                read_config()
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            configure();
            read_config()
        }
    }
}

fn configure() {
    let token = read_dialog("Vultr API token", "");
    let region = read_dialog("Default region", "New York");
    let output = read_dialog("Default output", "JSON");

    let conf_toml = toml::to_string(&Config::new(token, region, output)).unwrap();

    let mut conf_file = File::create(".vultr").unwrap();
    conf_file.write_all(conf_toml.as_bytes());
}

fn main() {
    let mut cli = App::new("vultr")
        .about("Vultr cli")
        .subcommand(vc2::cli())
        .subcommand(SubCommand::with_name("configure"));
    let matches = cli.clone().get_matches();

    match matches.subcommand() {
        ("configure", _) => configure(),
        ("vc2", x) => vc2::handle(x),
        (_, _) => {
            cli.print_long_help().unwrap();
        }
    }
}
