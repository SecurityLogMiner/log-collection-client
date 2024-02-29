use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal, Rect, Layout, Direction,
        Constraint
    },
    widgets::*,
};
use std::io::{self, stdout, Write, Result};

#[derive(Debug, Clone)]
struct Item {
    name: String,
    description: String,
}

impl Item {
    fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

pub fn 
show_menu() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut out = io::stdout();

    // put this in the state
    let items = vec![
        Item::new("Item 1", "Description for Item 1"),
        Item::new("Item 2", "Description for Item 2"),
        Item::new("Item 3", "Description for Item 3"),
    ];

    let mut selected_index = 0;

    loop {
        terminal.clear()?;
        terminal.draw(|frame| {
            for (i, item) in items.iter().enumerate() {
                if i == selected_index {
                    print!("> {} | ", item.name);
                } else {
                    print!("  {} | ", item.name);
                }
            }

            let description = &items[selected_index].description;
            println!(" {}", description);

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(frame.size());
            frame.render_widget(
                Paragraph::new("Top")
                    .block(Block::new().borders(Borders::ALL)),
                layout[0]);
            frame.render_widget(
                Paragraph::new("Bottom")
                    .block(Block::new().borders(Borders::ALL)),
                layout[1]);
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('q')
            {
                break;
            }
            if key.code == KeyCode::Up {
                if selected_index > 0 {
                    selected_index -= 1;
                }
            }
            if key.code == KeyCode::Down {
                if selected_index < items.len() - 1 {
                    selected_index += 1;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
