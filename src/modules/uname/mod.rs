use bitflags::bitflags;

use super::Module;

bitflags! {
    pub struct Flags: u32 {
        const NONE = 0;
        const KERNEL_NAME = 1 << 0;
        const NODE_NAME = 1 << 1;
        const KERNEL_RELEASE = 1 << 2;
        const KERNEL_VERSION = 1 << 3;
        const MACHINE = 1 << 4;
        const OPERATING_SYSTEM = 1 << 5;
    }
}

#[derive(Default)]
pub struct UnameModule;

impl Module for UnameModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("uname")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Print certain system information.")
            .args(&[
                clap::Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Print all information."),
                clap::Arg::new("kernel-name")
                    .short('s')
                    .long("kernel-name")
                    .help("Print the kernel name."),
                clap::Arg::new("node-name")
                    .short('n')
                    .long("node-name")
                    .help("Print the nodename."),
                clap::Arg::new("kernel-release")
                    .short('r')
                    .long("kernel-release")
                    .help("Print the kernel release."),
                clap::Arg::new("kernel-version")
                    .short('v')
                    .long("kernel-version")
                    .help("Print the kernel version."),
                clap::Arg::new("machine")
                    .short('m')
                    .long("machine")
                    .help("Print the machine hardware name."),
                clap::Arg::new("operating-system")
                    .short('o')
                    .long("operating-system")
                    .help("Print the operating system."),
            ])
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let mut flags = match clap.is_present("all") {
            true => Flags::all(),
            false => Flags::empty(),
        };

        if clap.is_present("kernel-name") {
            flags.insert(Flags::KERNEL_NAME);
        }

        if clap.is_present("node-name") {
            flags.insert(Flags::NODE_NAME);
        }

        if clap.is_present("kernel-release") {
            flags.insert(Flags::KERNEL_RELEASE);
        }

        if clap.is_present("kernel-version") {
            flags.insert(Flags::KERNEL_VERSION);
        }

        if clap.is_present("machine") {
            flags.insert(Flags::MACHINE);
        }

        if clap.is_present("operating-system") {
            flags.insert(Flags::OPERATING_SYSTEM);
        }

        if flags.is_empty() {
            // No flags set, print all
            flags.insert(Flags::all());
        }

        let mut output = String::new();

        let sysinfo = nix::sys::utsname::uname().expect("Failed to get system information");

        if flags.contains(Flags::KERNEL_NAME) {
            let kernel_name = sysinfo.sysname().to_string_lossy();
            output.push_str(&kernel_name);
            output.push(' ');
        }

        if flags.contains(Flags::NODE_NAME) {
            let nodename = sysinfo.nodename().to_string_lossy();
            output.push_str(&nodename);
            output.push(' ');
        }

        if flags.contains(Flags::KERNEL_RELEASE) {
            let kernel_release = sysinfo.release().to_string_lossy();
            output.push_str(&kernel_release);
            output.push(' ');
        }

        if flags.contains(Flags::KERNEL_VERSION) {
            let kernel_version = sysinfo.version().to_string_lossy();
            output.push_str(&kernel_version);
            output.push(' ');
        }

        if flags.contains(Flags::MACHINE) {
            let machine = sysinfo.machine().to_string_lossy();
            output.push_str(&machine);
            output.push(' ');
        }

        if flags.contains(Flags::OPERATING_SYSTEM) {
            let operating_system = sysinfo.sysname().to_string_lossy();
            output.push_str(&operating_system);
            output.push(' ');
        }

        println!("{}", output);
    }
}
