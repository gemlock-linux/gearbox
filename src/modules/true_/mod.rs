use std::process::exit;

use super::Module;

#[derive(Default)]
pub struct TrueModule;

impl Module for TrueModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("true")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("A module that always exists successfully")
    }

    fn entry(&self, _clap: &clap::ArgMatches) {
        exit(0);
    }
}
