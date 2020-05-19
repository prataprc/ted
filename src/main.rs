#![feature(backtrace)]
#![feature(box_syntax)]

use dirs;
use simplelog;
use structopt::StructOpt;

use std::{ffi, fs, path};

use ted::{
    config::Config,
    err_at,
    event::{Event, Ted},
    location::Location,
    state::State,
    window::{Coord, Window},
    window_code::WindowCode,
    Error, Result,
};

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short = "u", long = "config", default_value = "")]
    toml_file: String,

    #[structopt(long = "log", default_value = "")]
    log_file: String,

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(long = "trace")]
    trace: bool,

    #[structopt(long = "stats")]
    stats: bool,

    files: Vec<String>,
}

fn main() {
    let opts = Opt::from_args();

    match init_logger(&opts) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }

    std::panic::set_hook(box |panic_info| {
        let mut strng = format!(
            "panic occured: {:?}",
            panic_info.payload().downcast_ref::<String>().unwrap()
        );
        strng.push_str(&format!("{}", std::backtrace::Backtrace::capture()));
        fs::write("ted-panic.out", strng.as_bytes()).unwrap();
    });

    match run(opts) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

fn init_logger(opts: &Opt) -> Result<()> {
    let home_dir = err_at!(
        Fatal,
        dirs::home_dir().ok_or(format!("can't find home-directory"))
    )?;
    let log_file: path::PathBuf = if opts.log_file.is_empty() {
        [home_dir, path::Path::new(".ted.log").to_path_buf()]
    } else {
        [home_dir, path::Path::new(&opts.log_file).to_path_buf()]
    }
    .iter()
    .collect();

    let level_filter = if opts.trace {
        simplelog::LevelFilter::Trace
    } else if opts.verbose {
        simplelog::LevelFilter::Debug
    } else {
        simplelog::LevelFilter::Info
    };

    let mut lcnf = simplelog::ConfigBuilder::new();
    lcnf.set_location_level(simplelog::LevelFilter::Error)
        .set_target_level(simplelog::LevelFilter::Off)
        .set_thread_mode(simplelog::ThreadLogMode::Both)
        .set_thread_level(simplelog::LevelFilter::Error)
        .set_time_to_local(true)
        .set_time_format("%Y-%m-%dT%H-%M-%S%.3f".to_string());

    let fs = err_at!(Fatal, fs::File::create(&log_file))?;
    err_at!(
        Fatal,
        simplelog::WriteLogger::init(level_filter, lcnf.build(), fs)
    )?;

    Ok(())
}

fn run(opts: Opt) -> Result<()> {
    let state = {
        let config: Config = Default::default();
        State::new(config)?
    };
    let w = {
        let coord = Coord::new(1, 1, state.tm.rows, state.tm.cols);
        Window::Code(WindowCode::new(coord))
    };

    let evnt = if opts.files.len() == 0 {
        Event::Td(Ted::NewBuffer)
    } else {
        let mut flocs = vec![];
        for f in opts.files.clone().into_iter() {
            let f: ffi::OsString = f.into();
            flocs.push(Location::new_disk(&f));
        }
        Event::Td(Ted::OpenFiles { flocs })
    };

    state.event_loop(w, evnt)
}
