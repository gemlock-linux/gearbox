use clap::Command;
use modules::*;

mod modules;

struct Gearbox {
    modules: Vec<Box<dyn Module>>,
}

fn main() {
    let gearbox = Gearbox {
        modules: vec![
            Box::new(TouchModule::default()),
            Box::new(EchoModule::default()),
            Box::new(UnameModule::default()),
            Box::new(TrueModule::default()),
            Box::new(FalseModule::default()),
            Box::new(YesModule::default()),
            Box::new(PWDModule::default()),
            Box::new(WhoamiModule::default()),
            Box::new(SleepModule::default()),
            Box::new(NiceModule::default()),
        ],
    };

    let clap = Command::new("Gearbox")
        .version("v0.1.0")
        .author("Robin Alexander Plate")
        .about("A that merged multiple core utilities into one tool.")
        .subcommands(
            gearbox
                .modules
                .iter()
                .map(|module| module.command())
                .collect::<Vec<_>>(),
        )
        .arg_required_else_help(true)
        .get_matches();

    gearbox
        .modules
        .iter()
        .find(|module| module.command().get_name() == clap.subcommand().unwrap().0)
        .unwrap()
        .entry(clap.subcommand().unwrap().1);
}
