//! training-rust-logger, named simplelog
//!
//! !! This crate is not meant for production use. It is
//! personal Rust training project. If it helps you in some way
//! then feel free to use it. You have been warned. !!
//!
//! Library for simple logging functionality with multi-threading support.
//!
//! This crate uses logging approach where the [Backend](../simplelog/backend/struct.Backend.html) is started
//! in background thread. Each [Logger](../simplelog/struct.Logger.html) instance talks to the backend
//! over crossbeam::channel::unbounded channels and the backend
//! is responsible of doing the logging. This makes for single-writer
//! situation, which works well with stdout and file-logging.
//!
//! # Version control
//!
//! Version control is available at Github: [https://github.com/mpuromaki/training-rust-logger](https://github.com/mpuromaki/training-rust-logger)
//!
//! # Examples
//!
//! Create logger to stdout and send messages.
//!
//! ```rust
//! // First create and run the logging backend
//! // with configuration to log to stdout.
//! let backend_channel = simplelog::Backend::new()
//!     .name("logging-test-backend")
//!     .to_stdout()
//!     .build();
//!
//! // Then create local Logger instance to start
//! // logging. The LogLevel is set to LogLevel::Info
//! // to allow all messages with priority Info or
//! // higher to be logged.
//! let logger = simplelog::Logger::new(
//!    String::from("logging-test-frontend"),
//!    simplelog::LogLevel::Info,
//!    &backend_channel,
//! );
//!
//! // Create DEBUG message. This will be filtered out.
//! logger.debug("Message which will never be received.");
//!
//! // Create WARN message. This will be passed on to Backend.
//! logger.warn("Message which will be printed to stdout.");
//! // logging-test-frontend - WARN - {time} - Message which will be printed to stdout.
//!
//! ```

pub mod backend;
pub mod types;

pub use backend::Backend;
use chrono::prelude::*;
pub use types::{LogLevel, LoggingChannel, LoggingMessage};

pub struct Logger {
    name: String,
    loglevel: LogLevel,
    log: LoggingChannel,
}

/// Logger client.
/// Instances of Logger are used to send and filter messages to Backend.
impl Logger {
    /// Create new logger instance. This requires already defined Backend for LoggingChannel.
    pub fn new(name: String, loglevel: LogLevel, logging_channel: &LoggingChannel) -> Logger {
        Logger {
            name: name,
            loglevel: loglevel,
            log: logging_channel.clone(),
        }
    }

    /// Create and send Debug message.
    pub fn debug<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        if i32::from(&LogLevel::Debug) >= i32::from(&self.loglevel) {
            self.log
                .channel
                .send(LoggingMessage::Debug {
                    from: self.name.clone(),
                    when: Utc::now(),
                    msg: msg.into(),
                })
                .unwrap();
        }
    }

    /// Create and send Info message.
    pub fn info<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        if i32::from(&LogLevel::Info) >= i32::from(&self.loglevel) {
            self.log
                .channel
                .send(LoggingMessage::Info {
                    from: self.name.clone(),
                    when: Utc::now(),
                    msg: msg.into(),
                })
                .unwrap();
        }
    }

    /// Create and send Warning message.
    pub fn warn<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        if i32::from(&LogLevel::Warn) >= i32::from(&self.loglevel) {
            self.log
                .channel
                .send(LoggingMessage::Warn {
                    from: self.name.clone(),
                    when: Utc::now(),
                    msg: msg.into(),
                })
                .unwrap();
        }
    }

    /// Create and send Error message.
    pub fn error<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        if i32::from(&LogLevel::Error) >= i32::from(&self.loglevel) {
            self.log
                .channel
                .send(LoggingMessage::Error {
                    from: self.name.clone(),
                    when: Utc::now(),
                    msg: msg.into(),
                })
                .unwrap();
        }
    }

    /// Create and send Fatal message.
    pub fn fatal<S>(&self, msg: S)
    where
        S: Into<String>,
    {
        if i32::from(&LogLevel::Fatal) >= i32::from(&self.loglevel) {
            self.log
                .channel
                .send(LoggingMessage::Fatal {
                    from: self.name.clone(),
                    when: Utc::now(),
                    msg: msg.into(),
                })
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn log_message_to_channel() {
        use super::*;
        use std::time::Duration;

        let (log_transmit_channel, log_receiver_channel) =
            crossbeam::channel::unbounded::<String>();

        let backend_channel = Backend::new()
            .name("logging-test-backend")
            .to_channel(&log_transmit_channel)
            .build();

        let logger = Logger::new(
            String::from("logging-test-frontend"),
            LogLevel::Debug,
            &backend_channel,
        );

        // Send message
        logger.debug("TEST");

        // Check for message from logging-backend
        let received = log_receiver_channel
            .recv_timeout(Duration::new(2, 0))
            .expect("Log message receive failed.");

        // Check that we received the same message as we sent.
        // On TEST side we need to reverse the chars to get same as
        // received. This is to disregard any time etc data on the
        // logging message..
        // On the received side we need to reverse the message order,
        // skip the first character \n and then take 4..
        assert_eq!(
            String::from("TEST").chars().rev().collect::<Vec<char>>(),
            received
                .chars()
                .rev()
                .skip(1)
                .take(4)
                .collect::<Vec<char>>()
        );
    }

    #[test]
    fn log_message_to_stdout() {
        use super::*;
        use std::thread::sleep;
        use std::time::Duration;

        let backend_channel = Backend::new()
            .name("logging-test-backend")
            .to_stdout()
            .build();

        let logger = Logger::new(
            String::from("logging-test-frontend"),
            LogLevel::Debug,
            &backend_channel,
        );

        // Send message
        logger.debug("Test logging to STDOUT.");

        // Sleep for a while
        sleep(Duration::new(2, 0));
    }

    #[test]
    fn log_message_filter() {
        use super::*;
        use std::time::Duration;

        let (log_transmit_channel, log_receiver_channel) =
            crossbeam::channel::unbounded::<String>();

        let backend_channel = Backend::new()
            .name("logging-test-backend")
            .to_channel(&log_transmit_channel)
            .build();

        let logger = Logger::new(
            String::from("logging-test-frontend"),
            LogLevel::Warn,
            &backend_channel,
        );

        // Send message
        logger.debug("TEST");

        // We should NOT receive any messages
        let received = log_receiver_channel.recv_timeout(Duration::new(2, 0));
        match received {
            Ok(_) => panic!("Message received eventhough it should've been filtered."),
            Err(e) => assert_eq!(e, crossbeam::channel::RecvTimeoutError::Timeout),
        }
    }
}
