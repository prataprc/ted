#![feature(backtrace)]
#![feature(box_syntax)]

use structopt::StructOpt;

use std::{fs, process};

use ted::state::{Opt, State};

fn main() {
    let opts = Opt::from_args();

    std::panic::set_hook(box |panic_info| {
        let mut strng = format!(
            "panic occured: {:?}",
            panic_info.payload().downcast_ref::<String>().unwrap()
        );
        strng.push_str(&format!("{}", std::backtrace::Backtrace::capture()));
        fs::write("ted-panic.out", strng.as_bytes()).unwrap();
    });

    match State::new(opts) {
        Ok(state) => match state.event_loop() {
            Ok(_) => process::exit(0),
            Err(err) => {
                println!("{}", err);
                process::exit(2);
            }
        },
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
