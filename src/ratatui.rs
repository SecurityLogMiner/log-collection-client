use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::*,
};
use std::io::{stdout, Result};



pub fn start_ui() -> Result<()> {

    // startup: Enable raw mode for the terminal, giving us fine control over user input
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut state = ListState::default();
    let items = ["Item 1", "Item 2", "Item 3"];
    let list = List::new(items)
        .block(Block::default().title("Log Collection Menu").borders(Borders::ALL))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    
    let text = Text::raw("Press 'q' to quit");
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    loop {
        let cloned_list = list.clone();
        let cloned_paragraph = paragraph.clone();

        terminal.draw(|frame: &mut Frame<'_>| {
            let area = frame.size();
            frame.render_widget(cloned_paragraph, area);
            frame.render_stateful_widget(
                cloned_list, area, &mut state);
        })?;


        // Handle the user input through keyevents. However, this approach where all the event handlers in one function is not ideal.
        // This approach is not scalable. I am thinking of implementing Component Architecture or The Elm Architecture (TEA) for TUI implementation
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        let next = state.selected().map_or(0, |i| i + 1);
                        state.select(Some(next.min(items.len() - 1)));
                    }
                    KeyCode::Up => {
                        let prev = state.selected().map_or(0, |i| i.saturating_sub(1));
                        state.select(Some(prev));
                    }
                    KeyCode::Enter => {
                        // Handle the Enter key press to confirm selection
                        break;
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}