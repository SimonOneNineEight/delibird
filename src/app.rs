use crate::{
    event::{AppEvent, Event, EventHandler},
    input::InputField,
    task::TaskList,
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub task_list: TaskList,
    /// Event handler.
    pub events: EventHandler,
    pub input: InputField,
    pub input_mode: InputMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            task_list: TaskList::new(),
            events: EventHandler::new(),
            input: InputField::default(),
            input_mode: InputMode::Normal,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
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
                    KeyCode::Enter => self.task_list.toggle_status(),
                    _ => {}
                }
            }
            InputMode::Editing => match key_event.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input.style_textarea(InputMode::Normal);
                }
                KeyCode::Enter => {
                    self.task_list
                        .add_task(self.input.textarea().lines()[0].clone());
                    self.input.clear();
                }
                _ => {
                    self.input.input(key_event);
                }
            },
        }
        Ok(())
    }

    pub fn toggle_edit(&self) {}

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
