#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

mod vc2;

use clap::App;
use clap::SubCommand;
use std::io::{self, Write};
use std::fs::File;

#[derive(Deserialize, Serialize)]
struct Config {
    access_token: String,
    default_region: String,
    default_output: String,
}

impl Config {
    pub fn new(token: String, region: String, output: String) -> Config {
        Config {
            access_token: token,
            default_region: region,
            default_output: output,
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

fn configure() {
    let token = read_dialog("Vultr API token", "");
    let region = read_dialog("Default region", "New York");
    let output = read_dialog("Default output", "JSON");

    let conf_toml = toml::to_string(&Config::new(token, region, output)).unwrap();

    let mut conf_file = File::create("~/.vultr").unwrap(); 
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
        (_,_) => {cli.print_long_help().unwrap();},
    }
}
