mod app;
mod event;
mod handler;
mod tui;
mod display;

use crate::ratatui::app::App;
use crate::ratatui::tui::Tui;
use crate::ratatui::app::AppResult;

use event::{Event, EventHandler};
use handler::handle_key_events;
use std::io;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

/*
    Serves as the main function for the user interface.
    It initializes the terminal user interface, creates an application,
    and starts the main loop.

    Returns the result of the user interface.
*/
pub fn ratatui_main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
