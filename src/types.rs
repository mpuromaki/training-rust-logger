use chrono::prelude::{DateTime, Utc};
use crossbeam;

pub enum LoggingMessage {
    Debug {
        from: String,
        when: DateTime<Utc>,
        msg: String,
    },
    Info {
        from: String,
        when: DateTime<Utc>,
        msg: String,
    },
    Warn {
        from: String,
        when: DateTime<Utc>,
        msg: String,
    },
    Error {
        from: String,
        when: DateTime<Utc>,
        msg: String,
    },
    Fatal {
        from: String,
        when: DateTime<Utc>,
        msg: String,
    },
}

#[derive(Clone)]
pub struct LoggingChannel {
    pub channel: crossbeam::Sender<LoggingMessage>,
}

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl From<&LogLevel> for i32 {
    fn from(lvl: &LogLevel) -> i32 {
        match lvl {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warn => 2,
            LogLevel::Error => 3,
            LogLevel::Fatal => 4,
        }
    }
}
