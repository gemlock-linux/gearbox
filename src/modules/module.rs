pub trait Module {
    fn command(&self) -> clap::Command;
    fn entry(&self, clap: &clap::ArgMatches);
}
