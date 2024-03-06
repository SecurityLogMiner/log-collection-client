use ratatui::{
    layout::Alignment,
    style::{palette::material::{BLACK, YELLOW}, *},
    widgets::*,
    Frame,
};

use crate::ratatui::app::App;
use ratatui::prelude::*;

/*
    Renders the user interface through widgets
    and returns the result.
*/



// impl App {
//     fn render_title(&self, area: Rect, buf: &mut Buffer) {
//         Paragraph::new("Ratatui List Example")
//             .bold()
//             .centered()
//             .render(area, buf);
//     }

//     pub fn render_todo(&mut self, area: Rect, buf: &mut Buffer, app: &mut App, frame: &mut Frame) {
//         // We create two blocks, one is for the header (outer) and the other is for list (inner).
//         let outer_block = Block::default()
//             .borders(Borders::NONE)
//             .bg(BLACK)
//             .title("TODO List")
//             .title_alignment(Alignment::Center);
//         let inner_block = Block::default()
//             .borders(Borders::NONE)
//             .fg(BLACK);

//         // We get the inner area from outer_block. We'll use this area later to render the table.
//         let outer_area = area;
//         let inner_area = outer_block.inner(outer_area);

//         // We can render the header in outer_area.
//         outer_block.render(outer_area, buf);

//         // Iterate through all elements in the `items` and stylize them.
//         let list = List::new(app.items.items.clone())
//         .block(
//             Block::default()
//             .borders(Borders::ALL)
//             .title("Log Collection Menu")
//             .title_alignment(Alignment::Center)
//             .border_type(BorderType::Rounded),
//         )
//         .style(Style::default().fg(Color::Cyan).bg(Color::Black))
//         .highlight_style(Style::default().bg(Color::Yellow));

//         // Create a List from all list items and highlight the currently selected one

//         // We can now render the item list
//         // (look careful we are using StatefulWidget's render.)
//         // ratatui::widgets::StatefulWidget::render as stateful_render
//         StatefulWidget::render(list, inner_area, buf, &mut app.state);
//     }
// }
fn render_title(area: Rect, buf: &mut Buffer) -> Paragraph {
    Paragraph::new("Ratatui List Example")
        .centered()
        .clone()
}

fn render_list<'a>(app: &mut App) -> List <'a>{
    List::new(app.items.items.clone())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Log Collection Menu")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .highlight_style(Style::default().bg(Color::Yellow))
}

pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let area = Rect::new(0, 0, 20, 10);
    let mut buf = Buffer::empty(area);
    let title: Paragraph = render_title(area, &mut buf); // Call the render_title function to get the Paragraph instance

    let list = render_list(app);
    
    // Render the list widget using the frame
    frame.render_stateful_widget(list, frame.size(), &mut app.state);
    frame.render_widget(title, area);
    
}

