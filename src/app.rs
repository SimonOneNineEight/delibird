use crate::{
    event::{AppEvent, Event, EventHandler},
    input::InputField,
    storage::Storage,
    task::TaskList,
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use std::{fs, path::PathBuf};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub show_helper_popup: bool,
    pub task_list: TaskList,
    pub events: EventHandler,
    pub input: InputField,
    pub input_mode: InputMode,
    pub storage: Storage,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Result<Self, String> {
        let file_path = Self::initialize_storage()?;
        let storage = Storage::new(file_path);

        let mut task_list = TaskList::new();

        if let Ok(tasks) = storage.load() {
            for task in tasks {
                task_list.task_list.push(task);
            }
        }

        Ok(Self {
            running: true,
            show_helper_popup: false,
            task_list,
            storage,
            events: EventHandler::new(),
            input: InputField::default(),
            input_mode: InputMode::Normal,
        })
    }
    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                AppEvent::AddTask(description) => self.task_list.add_task(description),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.input_mode {
            InputMode::Normal => {
                match key_event.code {
                    KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                    KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                        self.events.send(AppEvent::Quit)
                    }
                    // Other handlers you could add here.
                    KeyCode::Char('n') => {
                        self.input_mode = InputMode::Editing;
                        self.input.style_textarea(InputMode::Editing);
                    }
                    KeyCode::Char('j') => self.task_list.select_next(),
                    KeyCode::Char('k') => self.task_list.select_previous(),
                    KeyCode::Char('d') => self.task_list.delete_selected_task(),
                    KeyCode::Enter => self.toggle_task(),
                    KeyCode::Char('h') => self.show_helper_popup = true,
                    KeyCode::Esc if self.show_helper_popup => self.show_helper_popup = false,
                    _ => {}
                }
            }
            InputMode::Editing => match key_event.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input.style_textarea(InputMode::Normal);
                }
                KeyCode::Enter => {
                    self.add_task(self.input.textarea().lines()[0].clone());
                    self.input.clear();
                }
                _ => {
                    self.input.input(key_event);
                }
            },
        }
        Ok(())
    }

    pub fn initialize_storage() -> Result<PathBuf, String> {
        let config_dir = dirs::config_dir().ok_or("Connot find config directory")?;
        let app_dir = config_dir.join("delibird");

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)
                .map_err(|err| format!("Failed to create directory: {}", err))?;
        }

        Ok(app_dir.join("tasks.json"))
    }

    pub fn save_tasks(&self) -> Result<(), String> {
        self.storage.save(&self.task_list.task_list)
    }

    pub fn auto_save(&self) {
        if let Err(err) = self.save_tasks() {
            eprintln!("Failed to save task: {}", err);
        }
    }

    pub fn add_task(&mut self, description: String) {
        self.task_list.add_task(description);
        self.auto_save();
    }

    pub fn toggle_task(&mut self) {
        self.task_list.toggle_status();
        self.auto_save();
    }

    pub fn delete_selected_task(&mut self) {
        self.task_list.delete_selected_task();
        self.auto_save();
    }

    pub fn toggle_helper_popup(&mut self) {
        self.show_helper_popup = !self.show_helper_popup;
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
