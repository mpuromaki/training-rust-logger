# training-rust-logger - simplelog - NOT FOR PRODUCTION USE

[See Rustdoc automatically generated documentation. Hosted by Github Pages.](https://mpuromaki.github.io/training-rust-logger/simplelog/index.html)

Simple multithreading compatible logger made as a training project in Rust language.

## Functionality

[x] Logging backend in separate thread.  
[x] Multiple Logger instances with simple log commands.  
[x] Logging to stdout.  
[x] Logging to channel. Usable for example in tests.  
[ ] Logging to rotating files.  
[ ] Some kind of heartbeating to main thread?  

## Usage

```rust
// First create and run the logging backend
// with configuration to log to stdout.
let backend_channel = simplelog::Backend::new()
    .name("logging-test-backend")
    .to_stdout()
    .build();

// Then create local Logger instance to start
// logging. The LogLevel is set to LogLevel::Info
// to allow all messages with priority Info or
// higher to be logged.
let logger = simplelog::Logger::new(
   String::from("logging-test-frontend"),
   simplelog::LogLevel::Info,
   &backend_channel,
);

// Create DEBUG message. This will be filtered out.
logger.debug("Message which will never be received.");

// Create WARN message. This will be passed on to Backend.
logger.warn("Message which will be printed to stdout.");
// logging-test-frontend - WARN - {time} - Message which will be printed to stdout.
```

## Building

```sh
cargo test
cargo build
cargo doc --no-deps
mv /target/docs/* /rustdoc/*
```
