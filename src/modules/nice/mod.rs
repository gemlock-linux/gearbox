use nix::{libc, unistd};
use std::{env, ffi, process::exit};

use super::Module;

#[derive(Default)]
pub struct NiceModule;

impl Module for NiceModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("nice")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Run a command with it's nice value adjusted (process priority). Please note that negative values require elevated privileges.")
            .args (&[
                clap::Arg::new("niceness")
                    .short('n')
                    .long("niceness")
                    .required(false)
                    .allow_hyphen_values(true)
                    .takes_value(true)
                    .help("The niceness value to set. Defaults to 0. range: -20 to 19 where -20 is the highest process priority and 19 is the lowest.")
                    .default_value("0"),

                clap::Arg::new("command")
                    .index(1)
                    .required(false)
                    .help("The command to run."),

                clap::Arg::new("args")
                    .index(2)
                    .required(false)
                    .multiple_values(true)
                    .allow_hyphen_values(true)
                    .help("The arguments to pass to the command."),
            ])
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let niceness = clap
            .value_of("niceness")
            .unwrap()
            .parse::<i32>()
            .unwrap_or(0);
        let command = clap.value_of("command").unwrap_or("");
        let args = clap
            .values_of("args")
            .unwrap_or(clap::Values::default())
            .collect::<Vec<_>>();

        if niceness < -20 || niceness > 19 {
            eprintln!("Nice value must be between -20 and 19");
            exit(1);
        }

        if command == "" {
            // Print the current niceness of the current command
            let pid = nix::unistd::getpid();
            let current_niceness =
                unsafe { libc::getpriority(libc::PRIO_PROCESS, pid.as_raw() as u32) };

            println!("{}", current_niceness);
            exit(0);
        }

        // Lets replace the current process with the new one
        unsafe {
            // Set the niceness
            libc::setpriority(libc::PRIO_PROCESS, 0, niceness);

            // Execute the command
            let command = ffi::CString::new(command).unwrap();
            let mut args = args
                .iter()
                .map(|arg| ffi::CString::new(*arg).unwrap())
                .collect::<Vec<_>>();
            args.insert(0, command.clone());

            let env = env::vars()
                .map(|(key, val)| format!("{}={}", key, val))
                .map(|env| ffi::CString::new(env).unwrap())
                .collect::<Vec<_>>();

            unistd::execvpe(&command, &args, &env).unwrap();
        }
    }
}
