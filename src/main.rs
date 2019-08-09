extern crate chrono;
extern crate signal_hook;

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

fn action() {
    println!("Did something at {:?}", std::time::Instant::now());
}

fn do_stuff() {
    prolapse::do_things(2019, 8, 9, 53.8021, -2.3157, 90.0 + 5.0 / 6.0, 10000, &action);
}
