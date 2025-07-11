use std::io;

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    tui::Tui,
};

mod app;
mod event;
mod tui;
mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app: App = App::new();

    // Initialize the terminal user interface.
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(io::stdout());
    let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    let events: EventHandler = EventHandler::new(250);
    let mut tui: Tui<CrosstermBackend<io::Stdout>> = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => app.handle_key_events(key_event)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
