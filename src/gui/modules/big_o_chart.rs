use tui::{
    layout::Alignment,
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
        GraphType,
    },
    symbols,
    text::{
        Span
    }
};

const WINDOW: (f64, f64) = (0.0, 100.0);
const DATA: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];
const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];

pub fn draw_big_o_chart() -> Chart<'static> {
    let x_labels = vec![
        Span::styled(
            format!("{}", WINDOW.0),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            "{}",
            (WINDOW.0 + WINDOW.1) / 2.0
        )),
        Span::styled(
            format!("{}", WINDOW.1),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];

    let datasets = vec![
        Dataset::default()
            .name("O(1)")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&DATA),
        Dataset::default()
            .name("O(log n)")
            .marker(
                symbols::Marker::Dot
            )
            .style(Style::default().fg(Color::Yellow))
            .data(&DATA2),
        Dataset::default()
            .name("O(n)")
            .marker(
                symbols::Marker::Dot
            )
            .style(Style::default().fg(Color::LightBlue))
            .data(&DATA2),
        Dataset::default()
            .name("O(n log n)")
            .marker(
                symbols::Marker::Dot
            )
            .style(Style::default().fg(Color::LightGreen))
            .data(&DATA2),
        Dataset::default()
            .name("O(n^2)")
            .marker(
                symbols::Marker::Dot
            )
            .style(Style::default().fg(Color::Green))
            .data(&DATA2),
        Dataset::default()
            .name("O(2^n)")
            .marker(
                symbols::Marker::Dot
            )
            .style(Style::default().fg(Color::Red))
            .data(&DATA2),
        Dataset::default()
            .name("O(n!)")
            .marker(
                symbols::Marker::Dot
            )
            .style(Style::default().fg(Color::DarkGray))
            .data(&DATA2)
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
                .bounds([WINDOW.0, WINDOW.1])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Operations")
                .style(Style::default().fg(Color::Gray))
                .bounds([WINDOW.0, WINDOW.1])
                .labels(vec![
                    Span::styled(format!("{}", WINDOW.0), Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled(format!("{}", WINDOW.1), Style::default().add_modifier(Modifier::BOLD)),
                ]),
        )
}