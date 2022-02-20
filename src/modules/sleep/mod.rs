#[derive(Default)]
pub struct SleepModule;

// Some draft to see how this could be implemented:
//
// usage: sleep [OPTION]... [TIME]
//
// Example:
//          sleep 2s // Sleep for 2 seconds
//          sleep 2m // Sleep for 2 minutes
//          sleep 2h // Sleep for 2 hours
//          sleep 2d // Sleep for 2 days
//          sleep 2w // Sleep for 2 weeks
//          sleep 2M // Sleep for 2 months
//          sleep 2y // Sleep for 2 years
//          sleep 2.5s // Sleep for 2.5 seconds
//          sleep 0.1m // Sleep for 6 seconds

use std::time::Duration;

use super::Module;

impl Module for SleepModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("sleep")
            .version("v0.1.0")
            .author("Robin Alexander Plate")
            .about("Sleep for a given time.")
            .args(&[clap::Arg::new("time").index(1).required(true).help(
                r#"
The time to sleep for.
Can be a number followed by a unit (s, m, h, d, w, M, y).
If no unit is specified, seconds are assumed.
Example: 2s, 2m, 2h, 2d, 2w, 2M, 2y or 2.5s etc.
"#,
            )])
    }

    fn entry(&self, clap: &clap::ArgMatches) {
        let time = clap.value_of("time").unwrap();

        // f64 is used to allow for decimal values
        let num = time
            .chars()
            .filter(|c| c.is_digit(10) || *c == '.')
            .collect::<String>()
            .parse::<f64>()
            .unwrap();

        let duration = match time.chars().nth(time.len() - 1).unwrap() {
            's' => Duration::from_secs_f64(num),
            'm' => Duration::from_secs_f64(num * 60.0),
            'h' => Duration::from_secs_f64(num * 60.0 * 60.0),
            'd' => Duration::from_secs_f64(num * 60.0 * 60.0 * 24.0),
            'w' => Duration::from_secs_f64(num * 60.0 * 60.0 * 24.0 * 7.0),
            'M' => Duration::from_secs_f64(num * 60.0 * 60.0 * 24.0 * 30.0),
            'y' => Duration::from_secs_f64(num * 60.0 * 60.0 * 24.0 * 365.0),
            _ => Duration::from_secs_f64(num),
        };

        std::thread::sleep(duration);
    }
}
