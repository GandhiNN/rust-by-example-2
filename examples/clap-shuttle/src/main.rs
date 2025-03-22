use clap::{Parser, Subcommand, crate_name};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Helper,
}

fn main() {
    loop {
        let mut buf = String::from(crate_name!());

        std::io::stdin()
            .read_line(&mut buf)
            .expect("Could not parse stdin");
        let line = buf.trim();
        let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

        println!("{:?}", args);

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => match cli.cmd {
                Commands::Get { key } => get_something(key),
                Commands::Set { key, value } => set_something(key, value),
                Commands::Helper => show_help(),
            },
            Err(_) => {
                println!("That's not a valid command - use the help command if you are stuck")
            }
        }
    }
}

fn get_something(key: String) {
    println!("Getting something using {}!", key);
}

fn set_something(key: String, val: String) {
    println!("Setting something using {}, {}!", key, val);
}

fn show_help() {
    println!(
        r#"COMMANDS:
get <KEY> - Gets the value of a given key and displays it. If no key given, retrieves all values and displays them.
set <KEY> <VALUE> - Sets the value of a given key.
    Flags: --is-true
"#
    );
}
