//! Code taken from <https://github.com/rust-cli/env_logger/blob/main/examples/custom_logger.rs> <3

use std::io::Write;

use env_filter::{Builder, Filter};
use webhook::client::WebhookClient;

use log::{Log, Metadata, Record, SetLoggerError};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const FILTER_ENV: &str = "RUST_LOG";
const LOG_FILE: &str = "shiba.log";

pub struct Logger {
    inner: Filter,
}

impl Logger {
    fn new() -> Self {
        let mut builder = Builder::from_env(FILTER_ENV);

        Self {
            inner: builder.build(),
        }
    }

    pub fn init() -> Result<(), SetLoggerError> {
        let logger = Self::new();

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.inner.matches(record) {
            macro_rules! set_stdout_color {
                ($r: expr, $g: expr, $b: expr, $stdout: ident) => {
                    $stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Rgb($r, $g, $b))))
                        .unwrap()
                };
            }

            let timestamp = chrono::Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            set_stdout_color!(131, 141, 140, stdout);
            write!(&mut stdout, "[").unwrap();

            stdout.reset().expect("Failed to reset stdout");

            write!(&mut stdout, "{timestamp}").unwrap();

            let mut str = format!(
                "[{timestamp} {} {}] {}",
                record.level(),
                record.module_path().unwrap_or_default(),
                record.args()
            );

            match record.level() {
                log::Level::Error => {
                    set_stdout_color!(255, 0, 0, stdout);
                    str.push_str(" <@1227590415755247657> <@1148282246462177401>");
                }
                log::Level::Warn => set_stdout_color!(255, 255, 0, stdout),
                log::Level::Info => set_stdout_color!(79, 184, 150, stdout),
                log::Level::Debug => set_stdout_color!(0, 255, 255, stdout),
                log::Level::Trace => set_stdout_color!(0, 0, 255, stdout),
            };
            write!(&mut stdout, " {} ", record.level()).unwrap();

            stdout.reset().expect("Failed to reset stdout");
            write!(&mut stdout, "{}", record.module_path().unwrap_or_default()).unwrap();

            set_stdout_color!(131, 141, 140, stdout);
            write!(&mut stdout, "] ").unwrap();

            stdout.reset().expect("Failed to reset stdout");
            write!(&mut stdout, "{}", record.args()).unwrap();

            if std::fs::metadata(LOG_FILE).is_err() {
                let _ = std::fs::File::create(LOG_FILE);
            }

            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(LOG_FILE)
                .expect("Failed to open log file");
            writeln!(file, "{str}").unwrap();
            println!();

            std::thread::spawn(move || {
                tokio::runtime::Runtime::new()
                    .expect("Failed to create tokio runtime")
                    .block_on(send_to_webhook(str));
            })
            .join()
            .expect("Failed to join send_to_webhook thread");
        }
    }

    fn flush(&self) {}
}

async fn send_to_webhook(str: String) {
    #[cfg(not(feature = "prod"))]
    let client = WebhookClient::new(
        &std::env::var("DEV_LOGGING_WEBHOOK_URL")
            .expect("You must provide a webhook URL to log to"),
    );
    #[cfg(feature = "prod")]
    let client = WebhookClient::new(
        &std::env::var("PROD_LOGGING_WEBHOOK_URL")
            .expect("You must provide a webhook URL to log to"),
    );

    client
        .send(|msg| msg.content(&str))
        .await
        .expect("Failed to send message to logging webhook");
}
