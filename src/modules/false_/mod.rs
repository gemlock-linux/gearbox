use std::process::exit;

use super::Module;

#[derive(Default)]
pub struct FalseModule;

impl Module for FalseModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("false").about("A module that always exists with failure")
    }

    fn entry(&self, _clap: &clap::ArgMatches) {
        exit(1);
    }
}
