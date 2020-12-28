pub mod dashboard;

use std::{
    io::Stdout,
    fmt::Error
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};
use crossterm::event::KeyCode;

use crate::gui::application::Application;
use crate::gui::store::Store;

pub trait Screenable {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<(), Error>;

    fn on_key(&mut self, key_code: KeyCode) {
        // do nothing
    }
}