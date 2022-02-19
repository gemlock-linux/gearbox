use std::env;

use super::Module;

#[derive(Default)]
pub struct PWDModule;

impl Module for PWDModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("pwd")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Print the current working directory.")
    }

    fn entry(&self, _clap: &clap::ArgMatches) {
        println!("{}", env::current_dir().unwrap().display());
    }
}
