#[derive(Debug)]
pub struct Logger {
    pub verbose: bool,
    pub debug: bool,
}

impl Logger {
    pub fn debug(&self, msg: &str) {
        if self.debug {
            println!("{}", msg)
        }
    }

    pub fn stdout(&self, msg: &str) {
        if self.verbose && !msg.is_empty() {
            print!("{}", msg)
        }
    }

    pub fn stderr(&self, msg: &str) {
        if self.verbose && !msg.is_empty() {
            eprint!("{}", msg)
        }
    }
}
