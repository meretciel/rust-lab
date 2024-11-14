
use log4rs::append::console::ConsoleAppender;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use chrono::Local;

pub fn init_logging(level: LevelFilter) {
    let pattern = "{d(%Y-%m-%d %H:%M:%S%.3f %Z)(local)}|{l}| {m}\n";

    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let dt = Local::now();
    let dt_string = dt.format("%Y%m%d_%H%M%S").to_string();
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(format!("/home/ryan/workspace/log/rust_application_{dt_string}.log"))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder()
            .appender("logfile")
            .appender("console")
            .build(level)).unwrap();

    log4rs::init_config(config).expect("Failed to initialize logger.");
}