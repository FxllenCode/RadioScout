use log::{debug, error, info, trace, warn};
use std::{time::SystemTime, env};


pub fn setup_logger(mut level: log::LevelFilter) -> Result<(), fern::InitError> {

    if env::var("RUST_LOG").is_ok() {
        level = match env::var("RUST_LOG").unwrap().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        };
    } else {
        let stringLevel = match level {
            log::LevelFilter::Trace => "trace",
            log::LevelFilter::Debug => "debug",
            log::LevelFilter::Info => "info",
            log::LevelFilter::Warn => "warn",
            log::LevelFilter::Error => "error",
            _ => "info",
        };
        env::set_var("RUST_LOG", stringLevel);
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}