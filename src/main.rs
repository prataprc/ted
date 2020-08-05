#![feature(backtrace)]
#![feature(box_syntax)]

use structopt::StructOpt;

use std::{fs, process};

use ted::state::{Opt, State};

fn main() {
    let opts = Opt::from_args();

    if opts.version {
        println!("dev {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    std::panic::set_hook(box |panic_info| {
        let s = match panic_info.payload().downcast_ref::<String>() {
            Some(s) => s.to_string(),
            None => "???".to_string(),
        };
        let mut strng = format!("panic occured: {}", s);
        strng.push_str(&format!("{}", std::backtrace::Backtrace::capture()));
        fs::write("ted-panic.out", strng.as_bytes()).unwrap();
    });

    // println!("{}", term_loop());

    match State::new(opts) {
        Ok(state) => match state.event_loop() {
            Ok(outs) => {
                println!("{}", outs);
                process::exit(0);
            }
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

#[allow(unused)]
fn term_loop() -> String {
    use crossterm::event::{read, Event, KeyCode, KeyEvent};
    use crossterm::execute;
    use crossterm::style::{style, PrintStyledContent};
    use std::io::{stdout, Write};
    use ted::term;
    use ted::util;

    let mut stats = util::Latency::new("EVENT");
    let mut r_stats = util::Latency::new("READT");

    let tm = term::Terminal::init();
    loop {
        let start = std::time::SystemTime::now();
        let evnt = read().unwrap();
        r_stats.sample(start.elapsed().unwrap());

        match evnt {
            Event::Key(KeyEvent { code, modifiers }) => match code {
                KeyCode::Char('q') => break,
                KeyCode::Char(ch) => {
                    //
                    execute!(stdout(), PrintStyledContent(style(ch.to_string())));
                }
                _ => (),
            },
            _ => (),
        }
    }
    format!("{}\n", r_stats.pretty_print())
}
