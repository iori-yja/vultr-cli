use clap::ArgMatches;
use clap::App;
use clap::SubCommand;

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

fn run(args: Option<&ArgMatches>) {
}

fn start(args: Option<&ArgMatches>) {
}

fn stop(args: Option<&ArgMatches>) {
}

fn terminate(args: Option<&ArgMatches>) {
}

fn reboot(args: Option<&ArgMatches>) {
}

fn describe(args: Option<&ArgMatches>) {
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
