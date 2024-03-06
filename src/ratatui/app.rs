use std::error;
use ratatui::widgets::*;

// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// Maximum number of menu items.
const MAX_MENU_ITEMS: u8 = 5;

// Application stateful list.
#[derive(Debug)]
#[derive(Clone)]

pub struct StatefulList {
    state: ListState,
    last_selected: Option<usize>,
    pub items: Vec<String>,
}

// Menu option struct.
struct MenuOption {
    name: &'static str,
    description: &'static str,
}

// Menu option implementation.
// Contains the name and description of the menu option.
impl MenuOption {
    fn new(name: &'static str, description: &'static str) -> Self {
        MenuOption { name, description }
    }
}

// Stateful list implementation.
// Contains the state of the list and the items in the list.
impl StatefulList {
    // Create stateful list with menu options.
    fn with_menu_options(items: [MenuOption; 4]) -> StatefulList{
        StatefulList {
            state: ListState::default(),
            items: items.iter().map(|option| format!("{}: {}", option.name, option.description)).collect(),
            last_selected: None,
        }
    }
}


// Application struct.
pub struct App{
    /// Is the application running?
    pub running: bool,
    /// Counter to keep track of menu position.
    pub counter: u8,
    /// Current state of the application.
    pub state: ListState,
    /// Items in the menu.
    pub items: StatefulList,
}

// Application implementation.
// Contains the state of the application and the items in the menu.
// The state of the application is used to keep track of the menu position.
impl<'a> App {
    // Default constructor for the application.
    fn default() -> Self {
        let menu_options = [
            MenuOption::new("DynamoDB", "AWS NoSQL database service"),
            MenuOption::new("IAM", "AWS Identity and Access Management"),
            MenuOption::new("KinesisFirehose", "AWS service for loading streaming data"),
            MenuOption::new("Exit", "Exit the program"),
        ];

        Self {
            running: true,
            counter: 0,
            state: ListState::default(),
            items: StatefulList::with_menu_options(menu_options), // Initialize the items with menu options
        }
    }

    // Constructs a new instance of `App`.
    pub fn new() -> Self {
        Self::default()
    }

    // Handles the tick event of the terminal.
    pub fn tick(&self) {
        // Handle tick event
    }

    // Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }


    // Navigate the menu up.
    // If the counter is greater than 0, it decrements the counter
    // and updates the selected menu item.
    pub fn navigate_menu_up(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
            let prev = self.state.selected().map_or(0, |i| i.saturating_sub(1));
            self.state.select(Some(prev));
            self.update_selected_menu_item();
        }
    }

    // Navigate the menu down.
    // If the counter is less than the maximum number of menu items,
    // it increments the counter and updates the selected menu item.
    pub fn navigate_menu_down(&mut self) {
        if self.counter < MAX_MENU_ITEMS {
            self.counter += 1;
            let next = self.state.selected().map_or(0, |i| i + 1);
            self.state.select(Some(next.min(self.items.items.len() as usize - 1)));
            self.update_selected_menu_item();
        }
    }

    // Select the current menu item.
    // If a menu item is selected, it prints its content.
    pub fn navigate_menu_select(&mut self) {
        if let Some(selected_item) = self.state.selected() {
            if let Some(content) = self.items.items.get(selected_item) {
                println!("{:?}", content);
            }
        }
    }

    // Update the selected menu item.
    pub fn update_selected_menu_item(&self) {
        match self.counter {
            0 => {
                // Handle selection 0
            }
            1 => {
                // Handle selection 1
            }
            2 => {
                // Handle selection 2
            }
            3 => {
                // Handle selection 3
            }
            4 => {
                // Handle selection 4
            }
            _ => {
                // Handle invalid selection
            }
        }
    }
}