use ratatui::{
    layout::Alignment,
    style::{palette::material::{BLACK, YELLOW}, *},
    widgets::*,
    Frame,
};

use crate::ratatui::app::App;
use ratatui::prelude::*;





fn render_list<'a>(app: &mut App) -> List <'a>{
    List::new(app.items.items.clone())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Log Collection Menu")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().bg(Color::Yellow))
}

pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let outer_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(35),
        Constraint::Percentage(65),
    ])
    .split(frame.size());

    let inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(25),
        Constraint::Percentage(75),
    ])
    .split(outer_layout[1]);

    let list = render_list(app);


    // Render the list widget using the frame
    frame.render_stateful_widget(
        list, 
        outer_layout[0], 
        &mut app.state);

    frame.render_widget(
        Paragraph::new("Top Nested Block")
            .block(Block::new()
            .borders(Borders::ALL)),
        inner_layout[0]);
    frame.render_widget(
            Paragraph::new("Bottom Nested Block")
                .block(Block::new()
                .borders(Borders::ALL)),
            inner_layout[1]);
}

