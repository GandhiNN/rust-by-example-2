use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn get_arguments() -> ArgMatches {
    Command::new("rcurl")
        .about("curl built in Rust")
        .version("1.0")
        .author("Ngakan Gandhi")
        .arg(Arg::new("url").index(1).required(true))
        .arg(
            Arg::new("x-method")
                .help("HTTP method which we want to use")
                .long("x-method")
                .short('x'),
        )
        .arg(
            Arg::new("data")
                .help("Payload we want to send with the request")
                .long("data")
                .short('d'),
        )
        .arg(
            Arg::new("headers")
                .help("Request header")
                .long("header")
                .short('e')
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("verbose")
                .help("verbose mode")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}
