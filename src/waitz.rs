use std::process::{Command, ExitStatus};
use std::{thread, time};

use crate::error::Result;
use crate::logger::Logger;

#[derive(Debug)]
pub struct Waitz<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
    pub interval: u64,
    pub no_retry: bool,
    pub logger: Logger,
}

impl Waitz<'_> {
    pub fn run(&self) {
        self.logger.debug(&format!("{:?}", &self));

        if self.no_retry {
            is_successful(self.command, &self.args, &self.logger);
            return;
        }

        while !is_successful(self.command, &self.args, &self.logger) {
            self.logger.verbose(&format!(
                "Wait for {:?} milliseconds to run again",
                self.interval
            ));
            thread::sleep(time::Duration::from_millis(self.interval));
        }
    }
}

fn is_successful(command: &str, args: &[&str], logger: &Logger) -> bool {
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

fn get_exit_code(command: &str, args: &[&str]) -> Result<ExitStatus> {
    let output = Command::new(command).args(args).output()?;
    Ok(output.status)
}
