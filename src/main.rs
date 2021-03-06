use std::time::Duration;

use clap::{App, Arg};

mod error;
mod logger;
mod waitz;

use crate::error::{Error, Result};
use crate::logger::Logger;
use crate::waitz::Waitz;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .short("i")
                .value_name("interval")
                .default_value("1000")
                .takes_value(true)
                .help("in which interval the command should be retried in milliseconds"),
        )
        .arg(
            Arg::with_name("no-retry")
                .long("no-retry")
                .short("n")
                .value_name("no-retry")
                .takes_value(false)
                .required(false)
                .help("Don't try to rerun the command in case it fails with non-zero exit code"),
        )
        .arg(
            Arg::with_name("max-retries")
                .long("max-retries")
                .short("r")
                .value_name("max-retries")
                .takes_value(true)
                .required(false)
                .default_value("0")
                .help("Positive integer how often the command should be retried"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .value_name("verbose")
                .takes_value(false)
                .required(false)
                .help("Forwards stdout/stderr from the command to the terminal"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .value_name("debug")
                .takes_value(false)
                .required(false)
                .help("Outputs debug information"),
        )
        .arg(
            Arg::with_name("COMMAND")
                .help("Which command should be waited for")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let verbose = matches.is_present("verbose");
    let debug = matches.is_present("debug");

    let logger = Logger { verbose, debug };

    logger.debug(&format!("Arguments: {:?}", matches));

    let interval = matches
        .value_of("interval")
        .ok_or_else(|| Error::NoneError("Could not get interval".to_owned()))?
        .parse::<u64>()?;

    let mut max_retries = matches
        .value_of("max-retries")
        .ok_or_else(|| Error::NoneError("Could not get max retries".to_owned()))?
        .parse::<u64>()?;

    // no-retry has precedence
    if matches.is_present("no-retry") {
        max_retries = 1
    }

    let mut original_args = matches
        .values_of("COMMAND")
        .ok_or_else(|| Error::NoneError("Could not get argguments for COMMAND".to_owned()))?;

    let command = original_args
        .next()
        .ok_or_else(|| Error::NoneError("Could not get command".to_owned()))?;

    let args: Vec<&str> = original_args.collect();

    let waitz = Waitz {
        command,
        args,
        interval: Duration::from_millis(interval),
        max_retries,
        logger,
    };

    waitz.run();
    Ok(())
}
