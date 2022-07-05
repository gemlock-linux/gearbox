use std::io::Read;

use sha1::{Digest, Sha1};

use super::Module;

#[derive(Default)]
pub struct Sha1sumModule;

impl Sha1sumModule {
    fn print_hash<S: AsRef<str>>(&self, sha1: Sha1, name: S) {
        let hash = sha1.finalize();
        let hash = format!("{:x}", hash);

        println!("{}  {}", hash, name.as_ref());
    }

    fn calculate_file<S: AsRef<str>>(&self, path: S) {
        let path = path.as_ref();

        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("{}: {}", path, err);
                return;
            }
        };

        let mut sha1 = Sha1::new();

        let mut buffer = [0u8; 1024];
        loop {
            let count = file.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }

            sha1.update(&buffer[..count]);
        }

        self.print_hash(sha1, path);
    }

    fn calculate_stdin(&self) {
        let mut sha1 = Sha1::new();
        let mut stdin = std::io::stdin();

        let mut buffer = [0u8; 1024];
        loop {
            let count = stdin.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }

            sha1.update(&buffer[..count]);
        }

        self.print_hash(sha1, "-");
    }
}

impl Module for Sha1sumModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("sha1sum")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Calculate the SHA1 checksum of a file (or stdin)") //DevSkim: ignore DS126858
            .args(&[clap::Arg::new("file")
                .index(1)
                .required(false)
                .multiple_values(true)
                .help("The file to calculate the checksum for ( use `-` for stdin)")])
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let path = clap.values_of("file");

        if let Some(s) = path {
            for path in s {
                if path == "-" {
                    self.calculate_stdin();
                } else {
                    self.calculate_file(path);
                }
            }
        } else {
            self.calculate_stdin();
        };
    }
}
