use super::types::{LoggingChannel, LoggingMessage};

use chrono::prelude::*;
use crossbeam;
use std::convert::From;
use std::time::Duration;
use std::{fs, path, thread};

fn create_log_line(msg: &LoggingMessage) -> String {
    match msg {
        LoggingMessage::Debug { from, when, msg } => {
            return format!("{} - {} - {} - {}\n", from, "DEBUG", when, msg)
        }
        LoggingMessage::Info { from, when, msg } => {
            return format!("{} - {} - {} - {}\n", from, "INFO", when, msg)
        }
        LoggingMessage::Warn { from, when, msg } => {
            return format!("{} - {} - {} - {}\n", from, "WARN", when, msg)
        }
        LoggingMessage::Error { from, when, msg } => {
            return format!("{} - {} - {} - {}\n", from, "ERROR", when, msg)
        }
        LoggingMessage::Fatal { from, when, msg } => {
            return format!("{} - {} - {} - {}\n", from, "FATAL", when, msg)
        }
    }
}

/// Backend functionality for logger.
///
/// Per software one instance of backend is configured,
/// which is responsible for creation of logging messages
/// for each logging client.
///
/// LoggingChannel type acts as a bridge between each
/// client and backend.
///
/// The channel used is unbounded channel from crossbeam.
///
/// ```rust
/// // Create logging backend.
/// let backend_channel = simplelog::Backend::new()
///     .name("logging-test-backend")
///     .to_stdout()
///     .build();
/// ```
/// # Panics
///
/// Yes, it does. This is not production ready.
///
#[derive(Debug, Clone)]
pub struct Backend {
    name: String,
    to_channel: Option<crossbeam::Sender<String>>,
    to_stdout: Option<()>,
    to_folder: Option<path::PathBuf>,
}

impl Backend {
    /// Create new backend. With default settings the backend
    /// is _NOT_ allowed to log anywhere. Please use any or
    /// all of the to_* functions.
    #[allow(dead_code)]
    pub fn new() -> Backend {
        Backend {
            name: String::from("Logging-backend"),
            to_channel: None,
            to_stdout: None,
            to_folder: None,
        }
    }

    /// Set the name of the background thread.
    #[allow(dead_code)]
    pub fn name(mut self, name: &str) -> Backend {
        self.name = name.to_owned();
        self
    }

    /// Add channel output (crossbeam::channel) to the logger.
    #[allow(dead_code)]
    pub fn to_channel(mut self, channel: &crossbeam::Sender<String>) -> Backend {
        self.to_channel = Some(channel.clone());
        self
    }

    /// Add standard output (cli output) to the logger.
    #[allow(dead_code)]
    pub fn to_stdout(mut self) -> Backend {
        self.to_stdout = Some(());
        self
    }

    /// Add file output to the logger.
    #[allow(dead_code)]
    pub fn to_folder(mut self, filepath: &path::PathBuf) -> Backend {
        unimplemented!();
        self.to_folder = Some(filepath.clone());
        self
    }

    /// Executes the backend and returns channel to it. Consumes Backend.
    #[allow(dead_code)]
    pub fn build(self) -> LoggingChannel {
        let (tx_to_log, rx_for_log) = crossbeam::channel::unbounded::<LoggingMessage>();

        // Start the logging thread
        let _backend_handle = thread::Builder::new()
            .name(self.name.clone())
            .spawn(move || backend_run(rx_for_log, self))
            .unwrap();

        // Return channel for any logging messages
        LoggingChannel { channel: tx_to_log }
    }
}

fn backend_run(channel: crossbeam::channel::Receiver<LoggingMessage>, settings: Backend) {
    // Get todays date
    let today = Utc::today();

    // If file logging defined, open the file in append mode.
    if settings.to_folder.is_some() {
        let mut filepath = settings.to_folder.clone().unwrap();
        filepath.push(format!("{}.log", today.format("%Y-%m-%d")));
        let mut fh = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath)
            .unwrap();
    }

    // Logger main loop, fetch messages and write to logfile
    loop {
        // -- Wait for message
        match channel.recv_timeout(Duration::from_millis(200)) {
            Ok(msg) => {
                if settings.to_stdout.is_some() {
                    // Send log line to stdout
                    println!("{}", create_log_line(&msg));
                }

                if settings.to_channel.is_some() {
                    // Send log line to channel
                    match &settings.to_channel {
                        Some(response_channel) => {
                            response_channel
                                .send(create_log_line(&msg))
                                .expect("Could not send to channel.");
                        }
                        None => (panic!("This should never happen.")),
                    }
                }

                if settings.to_folder.is_some() {
                    // Send log line to file
                }
            }
            Err(recv_timeout_error) => {
                // Not received any logs, heartbeat main
                // TODO: Check for date change and open new file if needed
                continue;
            }
        }
    }
}
