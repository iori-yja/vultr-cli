use clap::ArgMatches;
use clap::App;
use clap::SubCommand;

pub fn cli<'a>() -> App<'a,'a> {
    SubCommand::with_name("vc2")
        .about("Vultr Could Compute")
        .subcommand(SubCommand::with_name("run-instances"));
        .subcommand(SubCommand::with_name("describe-instances"));
        .subcommand(SubCommand::with_name("stop-instances"));
        .subcommand(SubCommand::with_name("terminate-instances"));
        .subcommand(SubCommand::with_name("reboot-instances"));
}

pub fn handle(args: Option<&ArgMatches>) {
}
