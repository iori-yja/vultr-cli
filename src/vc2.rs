use clap::ArgMatches;
use clap::App;
use clap::SubCommand;

pub fn cli<'a>() -> App<'a,'a> {
    SubCommand::with_name("vc2")
        .about("Vultr Could Compute")
}

pub fn handle(args: Option<&ArgMatches>) {
}
