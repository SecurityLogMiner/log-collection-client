use ratatui::{
    layout::Alignment,
    style::*,
    widgets::*,
    Frame,
};

use crate::ratatui::app::App;

// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples


        let list = List::new(app.items.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Log Collection Menu")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .highlight_style(Style::default().bg(Color::Yellow));
    frame.render_stateful_widget(list, frame.size(), &mut app.state);

    
}
