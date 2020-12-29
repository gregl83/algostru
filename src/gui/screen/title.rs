use std::{
    io::Stdout,
    rc::Rc,
    cell::RefCell,
    fmt::Error
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crossterm::event::KeyCode;

use crate::gui::store::Store;
use crate::gui::screen::Screenable;
use crate::gui::modules::welcome::draw_welcome_message;

pub struct Title {
    store: Rc<RefCell<Store>>,
}

impl Title {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Title {
            store
        }
    }
}

impl Screenable for Title {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<(), Error> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
            ].as_ref())
            .split(f.size());

        f.render_widget(
            draw_welcome_message(f.size().height),
            chunks[0]
        );

        Ok(())
    }

    fn on_key(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Left => {},
            KeyCode::Up => {},
            KeyCode::Down => {},
            KeyCode::Enter => {
                self.store.borrow_mut().welcomed = true;
            },
            _ => {}
        }
    }
}