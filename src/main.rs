#[allow(dead_code)]
mod util;
mod gui;
mod logger;

use std::{
    error::Error,
    io::{
        Stdout,
        Write,
        stdout,
    },
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

use log::{LevelFilter};
use crossterm::{
    event::{
        EnableMouseCapture,
        DisableMouseCapture,
        KeyCode,
    },
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::logger::Log;
use crate::gui::application::Application;
use crate::gui::store::Store;
use crate::gui::router::{
    Route,
    Router,
};
use crate::gui::screen::{
    Screenable,
    dashboard::Dashboard,
};
use crate::util::event::{
    Events,
    Event,
};

static LOGGER: Log = Log;

fn main() -> Result<(), Box<dyn Error>> {
    // todo - dynamic level filter (debug mode)
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug)).unwrap();

    // starting tui-rs + crossterm ---

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let store = Rc::new(RefCell::new(Store::new()));
    let router = Router::new(
        Rc::clone(&store),
        vec![
            (Route::Dashboard, Box::new(Dashboard::new(Rc::clone(&store)))),
        ]
    );
    let mut app = Application::new(Rc::clone(&store), router);

    terminal.clear()?;

    let shutdown = |mut terminal: Terminal<CrosstermBackend<Stdout>>| {
        disable_raw_mode();
        execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
        );
        terminal.show_cursor();
    };

    loop {
        terminal.draw(|f| app.draw(f))?;
        match events.next()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    shutdown(terminal);
                    break;
                }
                e => {
                    app.on_key(e);
                }
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            shutdown(terminal);
            break;
        }
    }

    Ok(())
}
