use dirs;
use simplelog;
use log::trace;
use structopt::StructOpt;
use crossterm::{
    self,
    cursor,
    event::{self as ct_event, Event as TermEvent},
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

use std::{path, ffi, result, io::{self, Write}, fs};

use kavi::{err_at, Error, view_port::Viewport};

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

struct Application {
    tm: Terminal,
    vp: Viewport,
}

impl Application {
    pub fn run(dir: &ffi::OsStr) -> Result<(), String> {
        let mut app = {
            let tm = Terminal::init()?;
            let vp = Viewport::new(0, 0, tm.rows, tm.cols);
            Application { tm, vp }
        };
        app.event_loop()
    }

    fn event_loop(mut self) -> Result<(), String> {
        loop {
            let evnt: TermEvent = err_at!(Fatal, ct_event::read())
                //
                .map_err(|e| e.to_string())?;
            trace!("Event-{:?}", evnt);
        }
    }
}

impl Application {
    #[inline]
    pub fn to_viewport(&self) -> Viewport {
        self.vp.clone()
    }
}


struct Terminal {
    stdout: io::Stdout,
    cols: u16,
    rows: u16,
}

impl Terminal {
    fn init() -> Result<Terminal, String> {
        let mut stdout = io::stdout();
        err_at!(
            //
            Fatal,
            terminal::enable_raw_mode()).map_err(|e| e.to_string()
        )?;
        err_at!(
            Fatal,
            execute!(
                stdout,
                EnterAlternateScreen,
                EnableMouseCapture,
                cursor::Hide
            )
        ).map_err(|e| e.to_string())?;

        let (cols, rows) = err_at!(
            //
            Fatal, terminal::size()).map_err(|e| e.to_string()
        )?;
        Ok(Terminal { stdout, cols, rows })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(
            self.stdout,
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

fn init_logger(opts: &Opt) -> Result<(), String> {
    if opts.log_file.is_empty() {
        Ok(())
    } else {
        let log_file: path::PathBuf = [
            dirs::home_dir().ok_or(format!("can't find home-directory"))?,
            path::Path::new(&opts.log_file).to_path_buf()
        ].iter().collect();

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

