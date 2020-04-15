use chrono;
use crossterm::{
    cursor,
    event::{self as ct_event, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{debug, info, trace};

use std::{
    ffi,
    io::{self, Write},
    mem,
};

use crate::{
    event::{self, Event},
    term_elements as te,
    term_layers::{self as tl, Layer},
};
use ledger::{
    core::{Error, Result, Store},
    err_at, util,
};

enum ViewFocus {
    Layer,
    Cmd,
}

struct View<S>
where
    S: Store,
{
    tm: Terminal,
    vp: te::Viewport,
    head: te::HeadLine,
    layers: Vec<Layer<S>>,
    status: te::StatusLine,
    // cmd: te::CmdLine,
    focus: ViewFocus,
    cursor: Option<(u16, u16)>,
}

impl<S> View<S>
where
    S: Store,
{
    fn new() -> Result<View<S>> {
        let tm = err_at!(Fatal, Terminal::init())?;
        // adjust full screen for a head-line in top and status-line at bottom.
        let vp = te::Viewport::new(1, 2, tm.rows - 2, tm.cols);

        debug!("App view-port {}", vp);

        Ok(View {
            tm,
            vp,
            head: Default::default(),
            layers: Default::default(),
            status: Default::default(),
            focus: ViewFocus::Layer,
            cursor: Some((1, 1)),
        })
    }

    #[inline]
    fn to_viewport(&self) -> te::Viewport {
        self.vp.clone()
    }
}

pub struct Application<S>
where
    S: Store,
{
    dir: ffi::OsString,
    view: View<S>,
    listeners: Vec<event::Tx>,
    store: Option<S>,
    date: chrono::Date<chrono::Local>,
    period: (chrono::Date<chrono::Local>, chrono::Date<chrono::Local>),
}

impl<S> AsRef<S> for Application<S>
where
    S: Store,
{
    fn as_ref(&self) -> &S {
        match &self.store {
            Some(store) => store,
            None => unreachable!(),
        }
    }
}

impl<S> AsMut<S> for Application<S>
where
    S: Store,
{
    fn as_mut(&mut self) -> &mut S {
        match &mut self.store {
            Some(store) => store,
            None => unreachable!(),
        }
    }
}

impl<S> Application<S>
where
    S: Store,
{
    pub fn run(dir: &ffi::OsStr) -> Result<()> {
        let mut app = Application {
            dir: dir.to_os_string(),
            view: View::new()?,
            listeners: Default::default(),
            store: Default::default(),
            date: chrono::Local::now().date(),
            period: util::date_to_period(chrono::Local::now().date()),
        };
        app.view.head = {
            let vp = te::Viewport::new(1, 1, 1, app.view.tm.cols);
            te::HeadLine::new(&mut app, vp)?
        };
        app.view.status = {
            let vp = te::Viewport::new(1, app.view.tm.rows, 1, app.view.tm.cols);
            te::StatusLine::new(&mut app, vp)?
        };

        match S::open(dir) {
            Ok(store) => {
                info!("Open workspace dir:{:?}", dir);
                app.store = Some(store);
                app.view.layers = vec![
                    //
                    Layer::OpenCompany(tl::OpenCompany::new(&mut app)?)
                ];
                Ok(())
            }
            Err(Error::NotFound(_)) => {
                info!("New workspace dir:{:?}", dir);
                app.view.layers = vec![
                    Layer::OpenCompany(tl::OpenCompany::new(&mut app)?),
                    Layer::NewWorkspace(tl::NewWorkspace::new(&mut app)?),
                ];
                Ok(())
            }
            Err(err) => Err(err),
        }?;

        app.event_loop()
    }

    fn event_loop(mut self) -> Result<()> {
        self.view.status.log("");
        self.refresh(true /*force*/)?.render()?;

        match self.view.layers.pop() {
            Some(mut layer) => {
                layer.focus(&mut self)?;
                self.view.layers.push(layer);
            }
            None => (),
        }

        loop {
            self.show_cursor()?;
            let evnt: Event = err_at!(Fatal, ct_event::read())?.into();

            trace!("Event-{:?}", evnt);

            let evnt = match evnt {
                Event::Resize { .. } => None,
                evnt => match self.view.layers.pop() {
                    Some(mut layer) => {
                        let evnt = layer.handle_event(&mut self, evnt.clone())?;
                        self.view.layers.push(layer);
                        evnt
                    },
                    None => Some(evnt)
                }
            };

            if let Some(evnt) = evnt {
                let m = evnt.to_modifiers();
                match evnt.to_key_code() {
                    Some(KeyCode::Char('q')) if m.is_empty() => break Ok(()),
                    _ => {
                        self.handle_event(evnt)?;
                    }
                }
            }

            err_at!(Fatal, execute!(self.view.tm.stdout, cursor::Hide))?;
            self.refresh(false /*force*/)?;
            err_at!(Fatal, self.view.tm.stdout.flush())?;
        }
    }

    fn handle_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        let m = evnt.to_modifiers();
        match evnt.to_key_code() {
            Some(KeyCode::Esc) if m.is_empty() && self.view.layers.len() > 1 => {
                self.view.layers.pop();
                self.refresh(true /*force*/)?.render()?;
                Ok(None)
            }
            _ => Ok(Some(evnt)),
        }
    }

    fn refresh(&mut self, force: bool) -> Result<&mut Self> {
        let mut head = mem::replace(&mut self.view.head, Default::default());
        head.refresh(self, force)?;
        self.view.head = head;

        let mut layers: Vec<Layer<S>> = self.view.layers.drain(..).collect();
        for layer in layers.iter_mut() {
            layer.refresh(self, force)?;
        }
        self.view.layers = layers;

        let mut status = mem::replace(&mut self.view.status, Default::default());
        status.refresh(self, force)?;
        self.view.status = status;

        Ok(self)
    }

    fn render(&mut self) -> Result<&mut Self> {
        err_at!(Fatal, queue!(self.view.tm.stdout, self.view.head))?;

        match self.view.layers.pop() {
            Some(layer) => {
                err_at!(Fatal, queue!(self.view.tm.stdout, layer))?;
                self.view.layers.push(layer);
            }
            None => (),
        }

        err_at!(Fatal, queue!(self.view.tm.stdout, self.view.status))?;

        err_at!(Fatal, self.view.tm.stdout.flush())?;

        Ok(self)
    }
}

impl<S> Application<S>
where
    S: Store,
{
    #[inline]
    pub fn to_viewport(&self) -> te::Viewport {
        self.view.to_viewport()
    }

    #[inline]
    pub fn as_mut_stdout(&mut self) -> &mut io::Stdout {
        &mut self.view.tm.stdout
    }

    #[inline]
    pub fn to_local_date(&self) -> chrono::Date<chrono::Local> {
        self.date.clone()
    }

    #[inline]
    pub fn to_local_period(&self) -> (chrono::Date<chrono::Local>, chrono::Date<chrono::Local>) {
        self.period.clone()
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        trace!("move cursor {:?}->None", cursor::position());
        self.view.cursor = None;

        Ok(())
    }

    pub fn move_cursor(&mut self, col: u16, row: u16) -> Result<()> {
        trace!("move cursor {:?}->{:?}", cursor::position(), (col, row));
        self.view.cursor = Some((col, row));

        Ok(())
    }

    pub fn subscribe(&mut self, tx: event::Tx) {
        self.listeners.push(tx)
    }

    pub fn publish(&mut self, evnt: Event) -> Result<()> {
        let mut listeners = vec![];
        for mut tx in self.listeners.drain(..) {
            if let Ok(_) = tx.send(evnt.clone()) {
                listeners.push(tx)
            }
        }
        self.listeners = listeners;
        Ok(())
    }

    #[inline]
    fn set_date(&mut self, date: chrono::Date<chrono::Local>) -> Result<&mut Self> {
        self.date = date;
        self.period = util::date_to_period(date);
        {
            self.publish(Event::Date(self.date.clone()))?;
            let (from, to) = self.period.clone();
            self.publish(Event::Period { from, to })?;
        }
        Ok(self)
    }

    #[inline]
    fn set_period(
        &mut self,
        period: (chrono::Date<chrono::Local>, chrono::Date<chrono::Local>),
    ) -> Result<&mut Self> {
        self.period = period;
        {
            let (from, to) = self.period.clone();
            self.publish(Event::Period { from, to })?;
        }
        Ok(self)
    }

    fn show_cursor(&mut self) -> Result<()> {
        match self.view.cursor {
            Some((col, row)) => err_at!(
                Fatal,
                execute!(
                    self.view.tm.stdout,
                    cursor::MoveTo(col - 1, row - 1),
                    cursor::EnableBlinking,
                    cursor::Show,
                )
            )?,
            None => err_at!(Fatal, execute!(self.view.tm.stdout, cursor::Hide,))?,
        }

        Ok(())
    }
}

struct Terminal {
    stdout: io::Stdout,
    cols: u16,
    rows: u16,
}

impl Terminal {
    fn init() -> Result<Terminal> {
        let mut stdout = io::stdout();
        err_at!(Fatal, terminal::enable_raw_mode())?;
        err_at!(
            Fatal,
            execute!(
                stdout,
                EnterAlternateScreen,
                EnableMouseCapture,
                cursor::Hide
            )
        )?;

        let (cols, rows) = err_at!(Fatal, terminal::size())?;
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
