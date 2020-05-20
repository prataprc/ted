#![feature(backtrace)]
#![feature(box_syntax)]

use std::{process, fs};

use ted::{state::State, Result};

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

    match State: new(opts) {
        Ok(state) => match state.event_loop() {
            Ok(_) => process::exit(0),
            Err(err) => {
                println!("{}", err);
                process:exit(2),
            }
        Err(err) => {
            println!("{}", err);
            process:exit(1),
        }
    }
}

fn run_code(opts: Opt) -> Result<()> {
    //use std::ffi;
    //use ted::{
    //    event::{Event, Ted},
    //    location::Location,
    //    window::{Coord, Window},
    //    window_code::WindowCode,
    //};

    //let w = {
    //    let coord = Coord::new(1, 1, state.tm.rows, state.tm.cols);
    //    Window::Code(WindowCode::new(coord))
    //};

    //let evnt = if opts.files.len() == 0 {
    //    Event::Td(Ted::NewBuffer)
    //} else {
    //    let mut flocs = vec![];
    //    for f in opts.files.clone().into_iter() {
    //        let f: ffi::OsString = f.into();
    //        flocs.push(Location::new_disk(&f));
    //    }
    //    Event::Td(Ted::OpenFiles { flocs })
    //};

    //state.event_loop(w, evnt)
    todo!()
}
