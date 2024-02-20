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
    symbols,
};
use std::{io::{stdout, Result}, sync::Arc};

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
      ])
      .split(r);
  
    Layout::default()
      .direction(Direction::Horizontal)
      .constraints([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
      ])
      .split(popup_layout[1])[1]
}

pub fn start_ui() -> Result<()> {

    // startup: Enable raw mode for the terminal, giving us fine control over user input
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    
    let mut state = ListState::default();
    let items = ["Kinesis Firehose", "DynamoDB", "IAM", "Exit"];
    let list = List::new(items)
        .block(Block::default().title("Log Collection Menu").borders(Borders::ALL))
        .highlight_style(Style::new().red().italic())
        .repeat_highlight_symbol(true)
        .style(Style::default().fg(Color::Yellow));
    
    let text = Text::raw("Press 'q' to quit");
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    loop {
        let cloned_list = list.clone();
        let cloned_paragraph = paragraph.clone();

        terminal.draw(|frame: &mut Frame<'_>| {
            // let area = frame.size();
            // frame.render_widget(cloned_paragraph, area[0]);
            let popup_area = centered_rect(frame.size(), 35, 35);
            frame.render_widget(Clear, popup_area);
            frame.render_stateful_widget(
                cloned_list, centered_rect(frame.size(), 100, 60), &mut state);
        })?;


        // Handle the user input through key events. However, this approach where all the event handlers in one function is not ideal.
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

// When building TUIs with ratatui, it’s vital to ensure that if your application encounters a panic, 
// it gracefully returns to the original terminal state. This prevents the terminal from getting stuck in a modified state, 
// which can be quite disruptive for users.
pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}