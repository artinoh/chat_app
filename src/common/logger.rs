use std::io::Write;
use log::Level;
use env_logger::fmt::Color;

pub fn init_logger(level: String) {
    let filter_level = string_to_log_level(&level);
    env_logger::Builder::new()
        .format(|buf, record| {
            let mut style = buf.style();
            match record.level() {
                Level::Error => style.set_color(Color::Red),
                Level::Warn => style.set_color(Color::Yellow),
                Level::Info => style.set_color(Color::Green),
                Level::Debug => style.set_color(Color::Blue),
                Level::Trace => style.set_color(Color::Magenta),
            };

            writeln!(
                buf,
                "{} {}:{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                style.value(record.level()),
                record.args()
            )
        })
        .filter(None, filter_level)
        .target(env_logger::Target::Stdout)
        .init();
}

fn string_to_log_level(level: &str) -> log::LevelFilter {
    match level {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    }
}