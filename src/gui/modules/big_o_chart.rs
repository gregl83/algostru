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

pub fn draw_big_o_chart<'a>(window: &[(f64, f64); 2], plot: &'a Plot) -> Chart<'a> {
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

    let datasets = vec![
        Dataset::default()
            .name("O(1)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&plot.lines[0].points),
        Dataset::default()
            .name("O(log n)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Yellow))
            .data(&plot.lines[0].points),
        Dataset::default()
            .name("O(n)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::LightBlue))
            .data(&plot.lines[0].points),
        Dataset::default()
            .name("O(n log n)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::LightGreen))
            .data(&plot.lines[0].points),
        Dataset::default()
            .name("O(n^2)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Green))
            .data(&plot.lines[0].points),
        Dataset::default()
            .name("O(2^n)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Red))
            .data(&plot.lines[0].points),
        Dataset::default()
            .name("O(n!)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::DarkGray))
            .data(&plot.lines[0].points)
    ];

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