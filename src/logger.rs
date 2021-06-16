pub struct Logger {
    pub verbose: bool,
}

impl Logger {
    pub fn verbose(&self, msg: &str) {
        if self.verbose {
            println!("{}", msg)
        }
    }
}
