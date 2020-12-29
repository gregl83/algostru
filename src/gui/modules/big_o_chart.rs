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
            format!("{}", -20.0),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            "{}",
            (-20.0 + 20.0) / 2.0
        )),
        Span::styled(
            format!("{}", 20.0),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];

    let datasets = vec![
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&DATA),
        Dataset::default()
            .name("data3")
            .marker(
                symbols::Marker::Dot
            )
            .style(Style::default().fg(Color::Yellow))
            .data(&DATA2),
    ];

    Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Chart",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 20.0])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([-20.0, 20.0])
                .labels(vec![
                    Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        )
}