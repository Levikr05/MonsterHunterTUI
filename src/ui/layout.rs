use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

pub fn ui(frame: &mut Frame) {
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(65),
        ])
        .split(frame.area());

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main[0]);

    let logs = Block::default().title("Logs").borders(Borders::ALL);
    let events = Block::default().title("Events").borders(Borders::ALL);
    let details = Block::default().title("Details").borders(Borders::ALL);

    frame.render_widget(logs, left[0]);
    frame.render_widget(events, left[1]);
    frame.render_widget(details, main[1]);
}