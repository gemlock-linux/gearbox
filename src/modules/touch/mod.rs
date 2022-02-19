use super::Module;

#[derive(Default)]
pub struct TouchModule;

impl Module for TouchModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("touch")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Create an empty file.")
            .arg(
                clap::Arg::new("file")
                    .index(1)
                    .required(true)
                    .help("The name of the file to create."),
            )
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let file = clap.value_of("file").unwrap();

        std::fs::File::create(file).unwrap();
    }
}
