use env_logger::fmt::Color;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logger() {
    // [2023-08-25T06:11:29Z INFO  bb_gamebot_rs]
    // env_logger::init();

    // [2023-08-25T09:45:24 INFO src/main.rs:48] logger initiated
    env_logger::Builder::new()
        .format(|buf, record| {
            let level = record.level();
            let mut style = buf.style();
            match record.level() {
                Level::Error => style.set_color(Color::Red),
                Level::Warn => style.set_color(Color::Yellow),
                Level::Info => style.set_color(Color::Green),
                Level::Debug => style.set_color(Color::Blue),
                Level::Trace => style.set_color(Color::Cyan),
            };
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                style.value(level),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    debug!("logger initiated");
}
