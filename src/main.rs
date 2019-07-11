#[macro_use]
extern crate log;

use chrono::Local;

use env_logger::Builder;

use log::LevelFilter;

use std::env;
use std::io::{self, Write};
use std::process;

use toml::Value;

use mdbook_html_taco::{ROOT_PATH, PRINT_PATH, STRIP_INDEX};
use mdbook_html_taco::errors::{Error, ErrorKind};
use mdbook_html_taco::renderer::{HtmlHandlebars, RenderContext, Renderer};

fn main() {
    init_logger();

    if let Err(e) = execute() {
        error!("{}", &e);
        process::exit(1);
    }
}

fn execute() -> Result<(), Error> {
    let ctx = RenderContext::from_json(io::stdin())?;

    if let Some(value) = ctx.config.get("output.html-taco.print-path") {
        match value {
            Value::String(value) => PRINT_PATH.set(value.into()).unwrap(),
            _ => return Err(ErrorKind::Msg("Configuration property 'output.html-taco.print-path' should be a string".into()).into())
        }
    } else {
        PRINT_PATH.set("print.md".into()).unwrap();
    }

    if let Some(value) = ctx.config.get("output.html-taco.strip-index") {
        match value {
            Value::Boolean(value) => STRIP_INDEX.set(value.clone()).unwrap(),
            _ => return Err(ErrorKind::Msg("Configuration property 'output.html-taco.strip-index' should be a boolean".into()).into())
        }
    } else {
        STRIP_INDEX.set(false).unwrap();
    }

    if let Some(value) = ctx.config.get("output.html-taco.root-path") {
        match value {
            Value::String(value) => {
                ROOT_PATH.set(value.into()).unwrap();

                HtmlHandlebars::new().render(&ctx)
            },
            _ => {
                Err(ErrorKind::Msg("Configuration property 'output.html-taco.root-path' should be a string".into()).into())
            }
        }
    } else {
        Err(ErrorKind::Msg("No absolute path root was configured".into()).into())
    }
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}
