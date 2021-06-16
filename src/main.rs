use clap::{App, Arg};
use std::io;
use std::process::{Command, ExitStatus};
use std::{thread, time};

mod logger;

fn main() {
    let matches = App::new("wait-for")
        .version("0.1.0")
        .author("Max Strübing <mxstrbng@gmail.com>")
        .about("Waits until the exit code of a program is zero")
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
            Arg::with_name("verbose")
                .long("verbose")
                .value_name("verbose")
                .takes_value(false)
                .required(false)
                .help("Outputs verbose information"),
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

    let interval = matches
        .value_of("interval")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let no_retry = matches.is_present("no-retry");

    let verbose = matches.is_present("verbose");
    let debug = matches.is_present("debug");
    let logger = logger::Logger { verbose, debug };

    logger.debug(&format!("Got args: {:?}", matches));

    let mut original_args = matches.values_of("COMMAND").unwrap();
    let command = original_args.next().unwrap();
    let args: Vec<&str> = original_args.collect();

    if no_retry {
        wait_for(command, &args, &logger);
        return;
    }

    while !wait_for(command, &args, &logger) {
        logger.verbose(&format!(
            "Wait for {:?} milliseconds to run again",
            interval
        ));
        thread::sleep(time::Duration::from_millis(interval));
    }
}

fn wait_for(command: &str, args: &[&str], logger: &logger::Logger) -> bool {
    logger.verbose(&format!("Running {} {}", command, args.join(" ")));
    let exit_code = get_exit_code(command, args);
    logger.debug(&format!("Got exit code: {:?}", exit_code));

    match exit_code {
        Ok(code) => {
            logger.verbose("Command exited successful");
            code.success()
        }
        Err(_err) => {
            logger.verbose("Command exited unsuccessful");
            false
        }
    }
}

fn get_exit_code(command: &str, args: &[&str]) -> Result<ExitStatus, io::Error> {
    let output = Command::new(command).args(args).output()?;
    Ok(output.status)
}
