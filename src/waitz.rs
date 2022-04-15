use std::process::{Command, Output};
use std::{str, thread, time::Duration};

use crate::error::Result;
use crate::logger::Logger;

#[derive(Debug)]
pub struct Waitz<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
    pub interval: Duration,
    pub max_retries: u64,
    pub logger: Logger,
}

impl Waitz<'_> {
    pub fn run(&self) {
        self.logger.debug(&format!("{:?}", &self));
        let mut runs = 0;

        while (self.max_retries == 0 || runs < self.max_retries)
            && !is_successful(self.command, &self.args, &self.logger)
        {
            runs += 1;

            self.logger.debug(&format!("Run {:?}", runs));
            self.logger
                .debug(&format!("Wait for {:?} to run again", self.interval));
            thread::sleep(self.interval);
        }
    }
}

fn is_successful(command: &str, args: &[&str], logger: &Logger) -> bool {
    logger.debug(&format!("Running {} {}", command, args.join(" ")));
    let result = run(command, args);
    logger.debug(&format!("Received: {:?}", result));

    match result {
        Ok(output) => {
            logger.stdout(std::str::from_utf8(&output.stdout).unwrap());
            logger.stderr(std::str::from_utf8(&output.stderr).unwrap());
            output.status.success()
        }
        Err(_err) => {
            logger.debug("Could not run command");
            false
        }
    }
}

fn run(command: &str, args: &[&str]) -> Result<Output> {
    Ok(Command::new(command).args(args).output()?)
}
