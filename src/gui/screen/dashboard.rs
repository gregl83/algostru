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
use std::borrow::{Borrow, BorrowMut};

const WINDOW: [(f64, f64); 2] = [
    (0.0, 100.0),
    (0.0, 1000.0)
];
const POINTS: u16 = 200;

pub struct Line {
    pub label: String,
    pub points: Vec<(f64, f64)>,
    y: fn(f64) -> f64
}

impl Line {
    pub fn push(&mut self, x: f64) {
        let y = (self.y)(x);
        if y <= WINDOW[1].1 {
            self.points.push((x, y));
        }
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

        let mut plot = Plot {
            lines: vec![
                Line {
                    label: String::from("O(1)"),
                    points: vec![],
                    y: |x| 1.0
                },
                Line {
                    label: String::from("O(log n)"),
                    points: vec![],
                    y: |x| x.log2()
                },
                Line {
                    label: String::from("O(n)"),
                    points: vec![],
                    y: |x| x
                },
                Line {
                    label: String::from("O(n log n)"),
                    points: vec![],
                    y: |x| x.mul(x.log2())
                },
                Line {
                    label: String::from("O(n^2)"),
                    points: vec![],
                    y: |x| x.powf(2.0)
                },
                Line {
                    label: String::from("O(2^n)"),
                    points: vec![],
                    y: |x| -> f64 {
                        let two: f64 = 2.0;
                        two.powf(x)
                    }
                },
                Line {
                    label: String::from("O(n!)"),
                    points: vec![],
                    y: |x| {
                        fn factorial(x: f64) -> f64 {
                            if x <= 1.0 { return 1.0; }
                            else { return (x) * factorial(x - 1.0); }
                        }
                        factorial(x)
                    }
                },
            ]
        };
        for i in 1..POINTS {
            let x = x_interval.mul(i as f64);
            for line in plot.lines.iter_mut() {
                line.push(x);
            }
        }

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
            draw_big_o_chart(&WINDOW, self.plot.borrow_mut()),
            chunks[0]
        );

        Ok(())
    }
}