use dirs;
use log::error;
use simplelog;
use structopt::StructOpt;

use std::{ffi, fs};

//mod app;
//mod edit_buffer;
//mod event;

// commands:
//      blinking, hide, show, enablemousecapture, disablemousecapture, clear, setsize,
//      resetcolor, setattribute, setattributes, setbackgroundcolor, setforegroundcolor, printstyledcontent, print
//      movedown, moveup, moveleft, moveright, moveto, movetocolumn, movetonextline, movetopreviousline,
//      restoreposition, saveposition
//      enteralternatescreen, leavealternatescreen,
//      scrolldown, scrollup,

#[derive(Debug, StructOpt)]
pub struct Opt {
    //#[structopt(long = "seed", default_value = "0")]
    //seed: u128,

    //#[structopt(long = "plot", default_value = "")]
    //plot: plot::PlotFiles,

    //#[structopt(long = "ignore-error", help = "Ignore log errors while plotting")]
    //ignore_error: bool,

    //#[structopt(long = "percentile", default_value = "99")]
    //percentile: String,

    #[structopt(long = "log", default_value="")]
    log_file: String,

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(long = "trace")]
    trace: bool,

    file: String,
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

    //let dir: &ffi::OsStr = opts.dir.as_ref();
    //match app::Application::<db_files::Db>::run(dir) {
    //    Ok(()) => (),
    //    Err(err) => error!("{}", err),
    //}
}

fn init_logger(opts: &Opt) -> Result<(), String> {
    if log_file.is_empty() {
        Ok(())
    } else {
        let log_file: path::PathBuf = [
            dirs::home_dir().map_err(|e| e.to_string())?,
            opts.file
        ].iter().collect();
        let log_file = log_file.into_os_file();

        let level_filter = if opts.trace {
            simplelog::LevelFilter::Trace
        } else if opts.verbose {
            simplelog::LevelFilter::Debug
        } else {
            simplelog::LevelFilter::Info
        };

        let mut config = simplelog::ConfigBuilder::new();
        config
            .set_location_level(simplelog::LevelFilter::Error)
            .set_target_level(simplelog::LevelFilter::Off)
            .set_thread_mode(simplelog::ThreadLogMode::Both)
            .set_thread_level(simplelog::LevelFilter::Error)
            .set_time_to_local(true)
            .set_time_format("%Y-%m-%dT%H-%M-%S%.3f".to_string());

        let fs = fs::File::create(&log_file).map_err(|e| e.to_string())?;
        simplelog::WriteLogger::init(level_filter, config.build(), fs).map_err(|e| e.to_string())?;

        Ok(())
    }
}
