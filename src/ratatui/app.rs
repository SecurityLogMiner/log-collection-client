use std::error;
use ratatui::widgets::*;
// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const MAX_MENU_ITEMS: u8 = 5;

// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    //Current state
    pub state: ListState,

    pub items: Vec<String>,

}


impl Default for App {
    fn default() -> Self {

        let items = vec![
            "DynamoDB".to_string(), 
            "Kinesis Firehose".to_string(), 
            "IAM".to_string()];

        Self {
            running: true,
            counter: 0,
            state: ListState::default(),
            items,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn navigate_menu_up(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
            let prev = self.state.selected().map_or(0, |i| i.saturating_sub(1));
            self.state.select(Some(prev));
            self.update_selected_menu_item();
        }
    }

    pub fn navigate_menu_down(&mut self) {
        if self.counter < MAX_MENU_ITEMS {
            self.counter += 1;
            let next = self.state.selected().map_or(0, |i| i + 1);
            self.state.select(Some(next.min(self.items.len() - 1)));
            self.update_selected_menu_item();
        }
    }

    pub fn navigate_menu_select(&mut self) {
        if let Some(selected_item) = self.state.selected() {
            if let Some(content) = self.items.get(selected_item as usize) {
                println!("{:?}", content);
            }
        }
    }

    pub fn update_selected_menu_item(&self) {
        match self.counter {
            0 => {
                // println!("Selected 0");
            }
            1 => {
                // println!("Selected 1");
            }
            2 => {
                // println!( "Selected 2");
            }
            3 => {
                // println!("Selected 3");
            }
            4 => {
                // println!("Selected 4");
            }
            _ => {
                // println!("Invalid");
            }
        }
    }
}
