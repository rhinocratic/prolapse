extern crate clap;
extern crate chrono;
extern crate signal_hook;

use std::process::Command;
use clap::{Arg, App};
use chrono::{Utc, SecondsFormat, Datelike};

pub const OFFICIAL: f64 = (90.0 + 5.0 / 6.0);
pub const CIVIL: f64 = 90.0;
pub const NAUTICAL: f64 = 102.0;
pub const ASTRONOMICAL: f64 = 108.0;

fn main() {
    let matches = App::new("prolapse")
        .arg(Arg::with_name("lat")
            .short("lat")
            .long("latitude")
            .help("The latitude of the camera in degrees")
            .required(true))
        .arg(Arg::with_name("lon")
            .short("lon")
            .long("longitude")
            .help("The longitude of the camera in degrees")
            .required(true))
        .arg(Arg::with_name("zenith")
            .short("zenith")
            .long("zenith")
            .help("The zenith for the desired type of sunset/sunrise - can be one of \"official\" (default), \"civil\", \"nautical\" or \"astronomical\"")
            .required(false))
        .arg(Arg::with_name("period")
            .short("p")
            .long("period_millis")
            .help("The period (in milliseconds) between shots.")
            .required(true))
        .get_matches();
    let now = Utc::now();
    let lat = matches.value_of("lat").unwrap().parse().unwrap();
    let lon = matches.value_of("lat").unwrap().parse().unwrap();
    let zenith = matches.value_of("zenith").unwrap().to_lowercase();
    let zenith = match zenith.as_ref() {
        "official" => OFFICIAL,
        "civil" => CIVIL,
        "nautical" => NAUTICAL,
        "astromomical" => ASTRONOMICAL,
        _ => OFFICIAL
    };
    let period = matches.value_of("period").unwrap().parse().unwrap();
    prolapse::do_things(now.year(), now.month(), now.day(), lat, lon, zenith, period, &action);
}

fn take_picture() {
    let fopt = format!("-o /home/pi/camera/{}.jpg", Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true));
    Command::new("raspistill")
        .arg(fopt)
        .spawn()
        .expect("Oh bollocks");
}

fn action() {
    take_picture();
}
