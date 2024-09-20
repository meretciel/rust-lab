use std::thread;
use std::time::Duration;
use log4rs::append::console::ConsoleAppender;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::time::{TimeTrigger, TimeTriggerConfig};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pattern = "{d(%Y-%m-%d %H:%M:%S%.3f %Z)(utc)}|{l}| {m}\n";
    // let logfile = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new(pattern)))
    //     .build("/home/ryan/workspace/tmp/rust_log_demo.log")?;

    let json_data = r#"
        {
            "interval": "3 seconds",
            "modulate": false,
            "max_random_delay": 1
        }
    "#;

    let tim_trigger_config: TimeTriggerConfig = serde_json::from_str(json_data).unwrap();
    let trigger = TimeTrigger::new(tim_trigger_config);

    let roller = FixedWindowRoller::builder()
        .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
        .build("/home/ryan/workspace/tmp/rust_log_demo.{}.log", 24)
        .unwrap();

    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build("/home/ryan/workspace/tmp/rust_log_demo.log", Box::new(policy))
        .unwrap();

    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder()
            .appender("logfile")
            .appender("console")
            .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    for k in 0..8 {
        log::info!("v2: Hello, world! {}", k);
        thread::sleep(Duration::from_secs(1))
    }
    Ok(())
}