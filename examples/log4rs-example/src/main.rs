use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main() {
    // Init logger from yaml config file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}
