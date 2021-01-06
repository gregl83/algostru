use tui::{
    style::{
        Color,
        Modifier,
        Style
    },
    widgets::{
        Axis,
        Block,
        Borders,
        Chart,
        Dataset,
    },
    symbols,
    text::{
        Span
    }
};

use crate::gui::screen::dashboard::Plot;

const COLORS: [Color; 7] = [
    Color::Green,
    Color::LightGreen,
    Color::LightYellow,
    Color::Yellow,
    Color::Rgb(255,165,0),
    Color::Rgb(255,140,0),
    Color::Red,
];

pub fn draw_big_o_chart<'a>(window: &[(f64, f64); 2], plot: &'a mut Plot) -> Chart<'a> {
    let x_labels = vec![
        Span::styled(format!("{}", window[0].0), Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!("{}", (window[0].0 + window[0].1) / 2.0)),
        Span::styled(format!("{}", window[0].1), Style::default().add_modifier(Modifier::BOLD)),
    ];

    let y_labels = vec![
        Span::styled(format!("{}", window[1].0), Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!("{}", (window[1].0 + window[1].1) / 2.0)),
        Span::styled(format!("{}", window[1].1), Style::default().add_modifier(Modifier::BOLD)),
    ];

    let mut datasets = vec![];
    for (i, line) in plot.lines.iter_mut().enumerate() {
        datasets.push(
            Dataset::default()
                .name(&line.label)
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(COLORS[i]))
                .data(&line.points)
        );
    }

    Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Big-O Notation",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Elements")
                .style(Style::default().fg(Color::Gray))
                .bounds([window[0].0, window[0].1])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Operations")
                .style(Style::default().fg(Color::Gray))
                .bounds([window[1].0, window[1].1])
                .labels(y_labels),
        )
}