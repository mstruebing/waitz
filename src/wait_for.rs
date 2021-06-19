use std::io;
use std::process::{Command, ExitStatus};
use std::{thread, time};

use crate::logger::Logger;

pub struct WaitFor<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
    pub interval: u64,
    pub no_retry: bool,
    pub logger: Logger,
}

impl WaitFor<'_> {
    pub fn run(&self) {
        if self.no_retry {
            wait_for(self.command, &self.args, &self.logger);
            return;
        }

        while !wait_for(self.command, &self.args, &self.logger) {
            self.logger.verbose(&format!(
                "Wait for {:?} milliseconds to run again",
                self.interval
            ));
            thread::sleep(time::Duration::from_millis(self.interval));
        }
    }
}

fn wait_for(command: &str, args: &[&str], logger: &Logger) -> bool {
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
