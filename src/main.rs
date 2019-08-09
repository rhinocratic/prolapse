extern crate astro;
extern crate chrono;
extern crate signal_hook;

use prolapse::*;
use std::process::Command;

fn main() {
    do_stuff();

    // let signal = unsafe { signal_hook::register(signal_hook::SIGALRM, || do_stuff()) };
    // match signal {
    //     Ok(s) => println!("{:?}", s),
    //     Err(e) => println!("{}", e),
    // };
    //
    // Command::new("ls")
    //     .arg("-l")
    //     .arg("-a")
    //     .spawn()
    //     .expect("Oh bollocks");
}

fn do_stuff() {}
