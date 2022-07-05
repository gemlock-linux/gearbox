use std::path::{Path, PathBuf};

use nix::sys::stat::Mode;

use super::Module;

#[derive(Default)]
pub struct MkDirModule;

impl MkDirModule {
    fn print_failure(&self, path: &str, reason: nix::errno::Errno) {
        let mut args = std::env::args();
        let arg0 = args.next().unwrap_or_default();
        let arg1 = args.next().unwrap_or_default();

        println!(
            "{}: cannot create directory '{}': {}",
            format!("{} {}", arg0, arg1),
            path,
            reason.desc()
        );
    }

    fn print_created(&self, path: &str) {
        let mut args = std::env::args();
        let arg0 = args.next().unwrap_or_default();
        let arg1 = args.next().unwrap_or_default();

        println!(
            "{}: created directory '{}'",
            format!("{} {}", arg0, arg1),
            path
        );
    }
}

impl Module for MkDirModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("mkdir")
            .about("A module to create directories")
            .args(&[
                clap::Arg::new("path")
                    .help("The path to the directory to create")
                    .required(true),
                clap::Arg::new("parents")
                    .help("Create parent directories as needed")
                    .short('p')
                    .long("parents")
                    .required(false),
                clap::Arg::new("mode")
                    .help("The mode to use for the directory")
                    .short('m')
                    .long("mode")
                    .required(false)
                    .takes_value(true),
                clap::Arg::new("verbose")
                    .help("Print the path of the directory that was created")
                    .short('v')
                    .long("verbose")
                    .required(false),
            ])
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let path = clap.value_of("path").unwrap();
        let parents = clap.is_present("parents");
        let mode = clap
            .value_of("mode")
            .map(|v| {
                let octal = u32::from_str_radix(v, 8).unwrap();
                Mode::from_bits_truncate(octal)
            })
            .unwrap_or(Mode::S_IRWXU | Mode::S_IRWXG | Mode::S_IRWXO);

        if !parents {
            match nix::unistd::mkdir(path, mode) {
                Ok(_) => {
                    if clap.is_present("verbose") {
                        self.print_created(path);
                    }
                }
                Err(e) => self.print_failure(path, e),
            }
        } else {
            let path = Path::new(path);
            let mut current_path = PathBuf::new();

            for dir in path.components() {
                current_path.push(dir);
                if current_path.exists() {
                    continue;
                }

                match nix::unistd::mkdir(&current_path, mode) {
                    Err(reason) => {
                        self.print_failure(current_path.to_str().unwrap(), reason);
                    }
                    Ok(_) => {
                        if clap.is_present("verbose") {
                            self.print_created(current_path.to_str().unwrap());
                        }
                    }
                }
            }
        }
    }
}
