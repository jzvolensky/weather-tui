use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Cell, Row, Table};


use tui::style::Style;
use tui::Terminal;

pub fn draw_weather_terminal(weather_data: Vec<(String, String)>) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(size);

        let rows = weather_data
            .into_iter()
            .map(|(key, value)| {
                let cells = vec![
                    Cell::from(key.to_string()),
                    Cell::from(value.to_string()),
                ];
                Row::new(cells)
            });

        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Weather Data").style(Style::default().bg(tui::style::Color::Black)))
            .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

        f.render_widget(table, chunks[0]);
    })?;

    //TODO: Figure out a way to fix the weird terminal reset output
    println!("\n");

    Ok(())
}
