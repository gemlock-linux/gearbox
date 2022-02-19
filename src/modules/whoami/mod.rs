use std::{ffi::CStr, process::exit};

use super::Module;

#[derive(Default)]
pub struct WhoamiModule;

impl Module for WhoamiModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("whoami")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Print the user's login name.")
    }

    fn entry(&self, _clap: &clap::ArgMatches) {
        let uid = nix::unistd::getuid();

        unsafe {
            let passwd = nix::libc::getpwuid(uid.as_raw());
            if passwd.is_null() {
                eprintln!("Error: Could not get user information.");
                exit(1);
            }

            let name = CStr::from_ptr((*passwd).pw_name);
            println!("{}", name.to_str().unwrap());
        }
    }
}
