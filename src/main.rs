extern crate clap;
extern crate chrono;

use std::process::Command;
use clap::{Arg, App};

pub const OFFICIAL: f64 = (90.0 + 5.0 / 6.0);
pub const CIVIL: f64 = 96.0;
pub const NAUTICAL: f64 = 102.0;
pub const ASTRONOMICAL: f64 = 108.0;

fn main() {
    let matches = App::new("prolapse")
        .version("1.0")
        .arg(Arg::with_name("lat")
            .short("l")
            .long("latitude")
            .help("The latitude of the camera in degrees")
            .takes_value(true)
            .allow_hyphen_values(true)
            .required(true))
        .arg(Arg::with_name("lon")
            .short("o")
            .long("longitude")
            .help("The longitude of the camera in degrees")
            .takes_value(true)
            .allow_hyphen_values(true)
            .required(true))
        .arg(Arg::with_name("zen")
            .short("z")
            .long("zenith")
            .help("The zenith for the desired type of sunset/sunrise - can be one of \"official\" (default), \"civil\", \"nautical\", \"astronomical\" or a custom angle (in degrees)")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("per")
            .short("p")
            .long("period")
            .help("The period (in milliseconds) between shots.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("action")
            .short("a")
            .long("action")
            .help("The (parameterless) command to be invoked.")
            .takes_value(true)
            .required(true))
        .get_matches();
    let lat = matches.value_of("lat").unwrap().parse().unwrap();
    let lon = matches.value_of("lon").unwrap().parse().unwrap();
    let zenith = matches.value_of("zen").unwrap().to_lowercase();
    let zenith = match zenith.as_ref() {
        "official" => OFFICIAL,
        "civil" => CIVIL,
        "nautical" => NAUTICAL,
        "astromomical" => ASTRONOMICAL,
        x => x.parse::<f64>().unwrap()
    };
    let period = matches.value_of("per").unwrap().parse().unwrap();
    let action = matches.value_of("action").unwrap();
    prolapse::schedule(lat, lon, zenith, period, &(|| act(action)));
}

fn act(action: &str) {
    Command::new(action)
        .spawn()
        .expect("Oh bollocks");
}
