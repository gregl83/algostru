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
use crate::gui::modules::big_o_chart::draw_big_o_chart;
use std::ops::Mul;
use std::borrow::Borrow;

const WINDOW: [(f64, f64); 2] = [
    (0.0, 100.0),
    (0.0, 1000.0)
];
const POINTS: u8 = 200;

pub struct Line {
    pub label: String,
    pub points: Vec<(f64, f64)>,
    pub y: fn(f64) -> f64
}

impl Line {
    pub fn push(&mut self, x: f64) {
        self.points.push(
            (x, (self.y)(x))
        );
    }
}

pub struct Plot {
    pub lines: Vec<Line>
}

pub struct Dashboard {
    store: Rc<RefCell<Store>>,
    plot: Plot,
}

impl Dashboard {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        let x_interval: f64 = WINDOW[0].1 / (POINTS as f64);

        let mut lines = vec![];
        let mut line = Line {
            label: String::from("O(1)"),
            points: vec![],
            y: |x| x.powf(2.0)
        };
        for i in 1..POINTS {
            let x = x_interval.mul(i as f64);

            line.push(x);
        }
        lines.push(line);
        let plot = Plot {
            lines,
        };

        Dashboard {
            store,
            plot,
        }
    }
}

impl Screenable for Dashboard {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<(), Error> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
            ].as_ref())
            .split(f.size());

        f.render_widget(
            draw_big_o_chart(&WINDOW, self.plot.borrow()),
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