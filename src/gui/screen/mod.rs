pub mod dashboard;
pub mod title;

use std::{
    io::Stdout,
    fmt::Error
};

use tui::{
    backend::CrosstermBackend,
    Frame,
};
use crossterm::event::KeyCode;

pub trait Screenable {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<(), Error>;

    fn on_key(&mut self, _key_code: KeyCode) {
        // do nothing
    }
}