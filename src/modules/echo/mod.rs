use super::Module;

#[derive(Default)]
pub struct EchoModule;

impl Module for EchoModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("echo")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Print a message to the standard output.")
            .arg(
                clap::Arg::new("message")
                    .index(1)
                    .required(true)
                    .help("The message to print."),
            )
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let message = clap.value_of("message").unwrap();

        println!("{}", message);
    }
}
