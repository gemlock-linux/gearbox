use super::Module;

#[derive(Default)]
pub struct YesModule;

impl Module for YesModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("yes")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Repeatedly output a line of text with the specified string or 'y'.")
            .args(&[clap::Arg::new("STRING")
                .help("The string to output. If not specified, output 'y'.")
                .multiple_occurrences(true)])
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let string = match clap.value_of("STRING") {
            Some(string) => string,
            None => "y",
        };

        loop {
            println!("{}", string);
        }
    }
}
