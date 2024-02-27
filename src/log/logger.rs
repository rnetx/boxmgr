use std::{error::Error, io, sync};

pub(crate) struct Logger {
    level: log::Level,
    writer: sync::Mutex<Box<dyn io::Write>>,
}

impl Logger {
    pub(crate) fn new(level: &str, writer: Box<dyn io::Write>) -> Result<Self, Box<dyn Error>> {
        let level = match level.to_ascii_lowercase().as_str() {
            "" => log::Level::Warn,
            "error" => log::Level::Error,
            "warn" | "warning" => log::Level::Warn,
            "info" => log::Level::Info,
            "debug" => log::Level::Debug,
            "trace" => log::Level::Trace,
            _ => {
                return Err(format!("invalid log level: {}", level).into());
            }
        };
        let writer = sync::Mutex::new(writer);
        let logger = Self { level, writer };
        Ok(logger)
    }

    pub(crate) fn set_global(self) -> Result<(), Box<dyn Error>> {
        log::set_max_level(self.level.to_level_filter());
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

#[derive(Debug, serde::Serialize)]
struct LogMessage {
    time: String,
    level: String,
    message: String,
}

fn level_to_string(level: &log::Level) -> String {
    match level {
        log::Level::Error => "error",
        log::Level::Warn => "warn",
        log::Level::Info => "info",
        log::Level::Debug => "debug",
        log::Level::Trace => "trace",
    }
    .to_owned()
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        let s = format!(
            "[{}] [{}] {}\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level_to_string(&record.level()),
            record.args().to_string().trim_end()
        );
        let _ = self.writer.lock().unwrap().write_all(s.as_bytes());
    }

    fn flush(&self) {
        let _ = self.writer.lock().unwrap().flush();
    }
}
